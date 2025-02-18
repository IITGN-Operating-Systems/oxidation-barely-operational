use stack_vec::StackVec;

use crate::console::{kprint, kprintln, CONSOLE};

/// Error type for `Command` parse failures.
#[derive(Debug)]
enum Error {
    Empty,
    TooManyArgs,
}

/// A structure representing a single shell command.
struct Command<'a> {
    args: StackVec<'a, &'a str>,
}

impl<'a> Command<'a> {
    /// Parse a command from a string `s` using `buf` as storage for the
    /// arguments.
    ///
    /// # Errors
    ///
    /// If `s` contains no arguments, returns `Error::Empty`. If there are more
    /// arguments than `buf` can hold, returns `Error::TooManyArgs`.
    fn parse(s: &'a str, buf: &'a mut [&'a str]) -> Result<Command<'a>, Error> {
        let mut args = StackVec::new(buf);
        for arg in s.split(' ').filter(|a| !a.is_empty()) {
            args.push(arg).map_err(|_| Error::TooManyArgs)?;
        }

        if args.is_empty() {
            return Err(Error::Empty);
        }

        Ok(Command { args })
    }

    /// Returns this command's path. This is equivalent to the first argument.
    fn path(&self) -> &str {
        self.args.get(0).unwrap()
    }
}

/// Starts a shell using `prefix` as the prefix for each line. This function
/// returns if the `exit` command is called.
pub fn shell(prefix: &str) -> ! {
    loop {
        // Print the prompt.
        kprint!("{}", prefix);

        // Buffer for user input (maximum 512 bytes).
        let mut line = [0u8; 512];
        let mut len = 0;

        // Read characters one by one.
        loop {
            let byte = {
                // Lock the console to read a byte.
                let mut console = CONSOLE.lock();
                console.read_byte()
            };

            match byte {
                b'\r' | b'\n' => {
                    // End of the line.
                    kprint!("\r\n");
                    break;
                }
                8 | 127 => {
                    // Backspace or delete.
                    if len > 0 {
                        len -= 1;
                        // Erase the character from the screen.
                        kprint!("\x08 \x08");
                    }
                }
                b => {
                    // Accept printable characters (ASCII 32 to 126).
                    if b >= 32 && b < 127 {
                        if len < line.len() {
                            line[len] = b;
                            len += 1;
                            // Echo the character.
                            kprint!("{}", b as char);
                        } else {
                            // If buffer is full, ring the bell.
                            kprint!("\x07");
                        }
                    } else {
                        // For any other non-visible character, ring the bell.
                        kprint!("\x07");
                    }
                }
            }
        }

        // Convert the input buffer to a string.
        let input = match core::str::from_utf8(&line[..len]) {
            Ok(s) => s,
            Err(_) => {
                kprintln!("error: invalid UTF-8");
                continue;
            }
        };

        // If the input is empty, print a new prompt.
        if input.trim().is_empty() {
            continue;
        }

        // Prepare storage for up to 64 command arguments.
        let mut arg_buf = [""; 64];
        let cmd = match Command::parse(input, &mut arg_buf) {
            Ok(cmd) => cmd,
            Err(Error::Empty) => continue,
            Err(Error::TooManyArgs) => {
                kprintln!("error: too many arguments");
                continue;
            }
        };

        // Check for built-in commands.
        match cmd.path() {
            "echo" => {
                // Echo prints all arguments (after the command name) with escape sequence processing
                let mut first = true;
                for arg in cmd.args.iter().skip(1) {
                    if !first {
                        kprint!(" ");
                    } 
                    first = false;
                    
                    // Process escape sequences in the argument
                    let mut chars = arg.chars().peekable();
                    while let Some(c) = chars.next() {
                        if c == '\\' {
                            if let Some(next_c) = chars.peek() {
                                match next_c {
                                    'n' => {
                                        kprint!("\n");
                                        chars.next(); // consume the 'n'
                                        continue;
                                    }
                                    't' => {
                                        kprint!("\t");
                                        chars.next(); // consume the 't'
                                        continue;
                                    }
                                    'r' => {
                                        kprint!("\r");
                                        chars.next(); // consume the 'r'
                                        continue;
                                    }
                                    '\\' => {
                                        kprint!("\\");
                                        chars.next(); // consume the second '\'
                                        continue;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        kprint!("{}", c);
                    }
                }
                kprintln!("");
            }
            "exit" => {
                kprintln!("exiting shell.");
                break;
            }
            unknown => {
                kprintln!("unknown command: {}", unknown);
            }
        }
    }
    // Since shell() should never return, loop forever.
    loop {}
}
