#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::cast_possible_truncation)]
#![no_main]
#![no_std]
#![feature(let_chains)]

extern crate alloc;

use widestring::U16CString;
use windows::core::BOOL;

use error::set_exit_code;

mod allocator;
mod error;
mod interop;
mod job;
mod resource;

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

unsafe fn main() -> windows::core::Result<()> {
    let resource = resource::ChildResource::load();

    if unsafe { interop::ris_windows_app(U16CString::from_ustr(resource.path.as_ustr()).unwrap()) }
    {
        windows::Win32::System::Console::FreeConsole()?;
    }

    let child = job::Job::new()?;
    let running_job = child.start(&resource)?;
    let exit_code = running_job.wait()?;

    set_exit_code(exit_code);

    Ok(())
}

#[no_mangle]
#[allow(clippy::similar_names)]
extern "C" fn wmain() -> u32 {
    match unsafe { main() } {
        Ok(()) => error::get_exit_code(),
        Err(e) => {
            _ = error::log_error(e.message());
            1
        }
    }
}
