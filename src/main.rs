#![warn(clippy::all, clippy::pedantic)]
#![no_main]
#![feature(let_chains)]

extern crate alloc;

use std::{
    error::Error,
    ffi::OsString,
    fs::File,
    io::{BufRead, BufReader, Read},
    path::PathBuf,
};

use libc::wchar_t;

const MAX_PATH: usize = windows::Win32::Foundation::MAX_PATH as usize + 2;

#[cfg(not(debug_assertions))]
#[no_mangle]
#[link_section = ".shim_path"]
pub static PATH: [wchar_t; MAX_PATH] = [0; MAX_PATH];

#[cfg(debug_assertions)]
pub static PATH: [wchar_t; MAX_PATH] = [
    67, 58, 92, 117, 115, 101, 114, 115, 92, 106, 117, 108, 105, 101, 92, 115, 99, 111, 111, 112,
    92, 97, 112, 112, 115, 92, 115, 102, 115, 117, 92, 99, 117, 114, 114, 101, 110, 116, 92, 115,
    102, 115, 117, 46, 101, 120, 101, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0,
];

#[no_mangle]
#[link_section = ".shim_args"]
pub static ARGS: [wchar_t; MAX_PATH] = [0; MAX_PATH];

#[no_mangle]
#[link_section = ".shim_command"]
pub static COMMAND: [wchar_t; 256] = [0; 256];

fn ctrl_handler(ctrl_type: u32) -> bool {
    use windows::Win32::System::Console::{
        CTRL_BREAK_EVENT, CTRL_CLOSE_EVENT, CTRL_C_EVENT, CTRL_LOGOFF_EVENT, CTRL_SHUTDOWN_EVENT,
    };

    matches!(
        ctrl_type,
        CTRL_C_EVENT
            | CTRL_CLOSE_EVENT
            | CTRL_LOGOFF_EVENT
            | CTRL_BREAK_EVENT
            | CTRL_SHUTDOWN_EVENT
    )
}

fn _main() -> Result<(), Box<dyn Error>> {
    Ok(())
}

#[no_mangle]
extern "C" fn main(_argc: isize, _argv: *const *const u8) -> isize {
    match _main() {
        Ok(()) => 0,
        Err(e) => {
            unsafe { libc::perror(e.to_string().as_ptr().cast()) };
            1
        }
    }
}
