mod parsers;

use serial;
use structopt;
use structopt_derive::StructOpt;
use xmodem::Xmodem;
use xmodem::Progress;
use std::path::PathBuf;
use std::time::Duration;

use structopt::StructOpt;
use serial::core::{CharSize, BaudRate, StopBits, FlowControl, SerialDevice, SerialPortSettings};

use parsers::{parse_width, parse_stop_bits, parse_flow_control, parse_baud_rate};

#[derive(StructOpt, Debug)]
#[structopt(about = "Write to TTY using the XMODEM protocol by default.")]
struct Opt {
    #[structopt(short = "i", help = "Input file (defaults to stdin if not set)", parse(from_os_str))]
    input: Option<PathBuf>,

    #[structopt(short = "b", long = "baud", parse(try_from_str = "parse_baud_rate"),
                help = "Set baud rate", default_value = "115200")]
    baud_rate: BaudRate,

    #[structopt(short = "t", long = "timeout", parse(try_from_str),
                help = "Set timeout in seconds", default_value = "10")]
    timeout: u64,

    #[structopt(short = "w", long = "width", parse(try_from_str = "parse_width"),
                help = "Set data character width in bits", default_value = "8")]
    char_width: CharSize,

    #[structopt(help = "Path to TTY device", parse(from_os_str))]
    tty_path: PathBuf,

    #[structopt(short = "f", long = "flow-control", parse(try_from_str = "parse_flow_control"),
                help = "Enable flow control ('hardware' or 'software')", default_value = "none")]
    flow_control: FlowControl,

    #[structopt(short = "s", long = "stop-bits", parse(try_from_str = "parse_stop_bits"),
                help = "Set number of stop bits", default_value = "1")]
    stop_bits: StopBits,

    #[structopt(short = "r", long = "raw", help = "Disable XMODEM")]
    raw: bool,
}

fn progress_fn(progress: Progress) {
    println!("Progress: {:?}", progress);
}

fn main() {
    use std::fs::File;
    use std::io::{self, BufReader};

    let opt = Opt::from_args();
    let mut port = serial::open(&opt.tty_path).expect("path points to invalid TTY");
    // FIXME: Implement the ttywrite utility.
    let mut port_config = port.read_settings().unwrap_or_else(|_| panic!("Failed to read port configuration"));

    port_config.set_baud_rate(connection.baud_speed).unwrap_or_else(|_| panic!("Invalid baud rate configuration"));
    port_config.set_char_size(connection.data_bits);
    port_config.set_stop_bits(connection.end_bits);
    port_config.set_flow_control(connection.flow_mode);

    port.apply_settings(&port_config).unwrap_or_else(|_| panic!("Couldn't configure serial interface"));
    port.set_communication_timeout(Duration::from_secs(connection.wait_limit))
        .expect("Timeout configuration error");

    // Initialize data source
    let mut data_source: Box<dyn io::Read> = match connection.source_file {
        Some(path) => {
            let input_stream = File::open(path)
                .unwrap_or_else(|_| panic!("Unable to access input file"));
            Box::new(BufReadContainer::new(input_stream))
        }
        None => Box::new(BufReadContainer::new(io::keyboard_input())),
    };

    // Execute data transfer
    let transfer_count = if connection.direct_mode {
        io::transfer_data(&mut data_source, &mut port)
            .expect("Data stream copy failure")
    } else {
        XmodemProtocol::send_data_with_status(&mut data_source, &mut port, transfer_update)
            .expect("XMODEM transmission failure") as u64
    };

    println!("Transferred {} bytes successfully", transfer_count);
}