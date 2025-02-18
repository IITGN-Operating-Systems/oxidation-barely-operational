#![feature(alloc_error_handler)]
#![feature(decl_macro)]
#![feature(auto_traits)]
#![feature(negative_impls)]
#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

#[cfg(not(test))]
mod init;

pub mod console;
pub mod mutex;
pub mod shell;

use console::kprintln;
use shell::shell;

// FIXME: You need to add dependencies here to
// test your drivers (Phase 2). Add them as needed.


#[no_mangle]
pub extern "C" fn kmain() -> ! {
    // FIXME: Start the shell.
    kprintln!("Welcome to the OS shell!");
    // Start the shell with the prompt "> "
    shell("> ");
}
