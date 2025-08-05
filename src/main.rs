#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::cast_possible_truncation)]
#![no_main]
#![no_std]
#![feature(rustc_private)]

extern crate compiler_builtins_but_not_named_that;

extern crate alloc;

use widestring::WideCString;
use windows::{
    Win32::{Foundation::GetLastError, System::Threading::ExitProcess},
    core::BOOL,
};

use common::exe_type::ExeType;
use error::{ExitCode, handle_windows_error};

mod allocator;
mod error;
mod job;
mod resource;

unsafe extern "system" fn ctrl_handler(ctrl_type: u32) -> BOOL {
    use windows::Win32::System::Console::{
        CTRL_BREAK_EVENT, CTRL_C_EVENT, CTRL_CLOSE_EVENT, CTRL_LOGOFF_EVENT, CTRL_SHUTDOWN_EVENT,
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
    unsafe {
        let resource = resource::ChildResource::load();

        if let Some(exe_type) =
            ExeType::from_path(WideCString::from_ustr_unchecked(resource.path.as_ustr()))
        {
            if exe_type.is_windows() {
                windows::Win32::System::Console::FreeConsole()?;
            }
        } else {
            error::log_error("Shim: Could not determine executable type.\n")?;
        }

        let child = job::Job::new()?;
        let running_job = child.start(&resource)?;
        let exit_code = running_job.wait()?;

        ExitCode::set_code(exit_code);
        if exit_code != 0 {
            ExitCode::set_reason(error::ExitCode::ChildError);
        }

        Ok(())
    }
}

#[unsafe(no_mangle)]
#[allow(clippy::similar_names)]
extern "C" fn entry() -> ! {
    match unsafe { main() } {
        Ok(()) => {
            if ExitCode::get_code() != 0 {
                let last_error = unsafe { GetLastError() };
                if last_error.0 != 0 {
                    handle_windows_error(last_error);
                }
                if let Some(message) = ExitCode::reason().message() {
                    _ = error::log_error(message);
                }
            }
        }
        #[allow(clippy::cast_sign_loss)]
        Err(e) => {
            _ = error::log_error(e.message());
            ExitCode::set_reason(error::ExitCode::Unknown(e.code().0 as u32));
        }
    }

    unsafe { ExitProcess(ExitCode::get_code()) }
}
