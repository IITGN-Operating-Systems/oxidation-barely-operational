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
use crate::shell::shell;

// FIXME: You need to add dependencies here to
// test your drivers (Phase 2). Add them as needed.

fn kmain() -> ! {
    // Start the shell with a prefix
    shell("shell> ");
    loop {}
}
