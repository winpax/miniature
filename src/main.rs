#![warn(clippy::all, clippy::pedantic)]
#![no_main]
#![no_std]
#![feature(let_chains)]

extern crate alloc;

use core::ffi::c_int;

use error::set_exit_code;
use libc::wchar_t;
use widestring::{U16CStr, WideString};
use windows::{core::BOOL, Win32::System::Environment::GetCommandLineW};

mod error;
mod job;

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
pub static COMMAND: [wchar_t; MAX_PATH] = [0; MAX_PATH];

unsafe extern "system" fn ctrl_handler(ctrl_type: u32) -> BOOL {
    use windows::Win32::System::Console::{
        CTRL_BREAK_EVENT, CTRL_CLOSE_EVENT, CTRL_C_EVENT, CTRL_LOGOFF_EVENT, CTRL_SHUTDOWN_EVENT,
    };

    let matched_ctrl = matches!(
        ctrl_type,
        CTRL_C_EVENT
            | CTRL_CLOSE_EVENT
            | CTRL_LOGOFF_EVENT
            | CTRL_BREAK_EVENT
            | CTRL_SHUTDOWN_EVENT
    );

    BOOL::from(matched_ctrl)
}

extern "C" {
    fn compute_program_length(commandline: *const wchar_t) -> c_int;
    fn is_windows_app(path: *const wchar_t) -> BOOL;
}

fn get_path() -> &'static U16CStr {
    U16CStr::from_slice_truncate(&PATH).unwrap()
}

fn get_args() -> &'static U16CStr {
    U16CStr::from_slice_truncate(&ARGS).unwrap()
}

unsafe fn calculate_command() -> windows::core::Result<WideString> {
    let mut command_length: usize = 256;
    let path = get_path();
    let args = get_args();

    command_length += path.len();
    command_length += args.len() + 1;

    let commandline = unsafe { GetCommandLineW() };

    let program_length = usize::try_from(unsafe { compute_program_length(commandline.as_ptr()) })?;

    let given_command = &unsafe { commandline.as_wide() }[program_length..];

    command_length += given_command.len();

    let mut command = WideString::with_capacity(command_length);
    command.push(path);
    command.push_char(' ');
    command.push(args);
    command.push_char(' ');
    command.push_slice(given_command);
    command.push_char(' ');

    Ok(command)
}

unsafe fn start() -> windows::core::Result<()> {
    let command = calculate_command()?;

    if unsafe { is_windows_app(get_path().as_ptr()) }.as_bool() {
        windows::Win32::System::Console::FreeConsole()?;
    }

    let child = job::Job::new()?;
    let running_job = child.start(command.as_ustr())?;
    let exit_code = running_job.wait()?;

    set_exit_code(exit_code);

    Ok(())
}

#[no_mangle]
extern "C" fn main(_argc: isize, _argv: *const *const u8) -> u32 {
    match unsafe { start() } {
        Ok(()) => error::get_exit_code(),
        Err(e) => {
            unsafe { libc::perror(e.message().as_ptr().cast()) };
            1
        }
    }
}
