use alloc::string::String;
use widestring::WideCString;
use windows::{
    Win32::System::Threading::{
        CREATE_SUSPENDED, CreateProcessW, PROCESS_INFORMATION, STARTUPINFOW,
    },
    core::{PCWSTR, PWSTR},
};

use crate::{error, resource::ChildResource};

pub trait Spawn {
    unsafe fn spawn_command(&self) -> windows::core::Result<PROCESS_INFORMATION>;
    unsafe fn spawn_shell(&self) -> windows::core::Result<PROCESS_INFORMATION>;
}

impl Spawn for ChildResource {
    unsafe fn spawn_command(&self) -> windows::core::Result<PROCESS_INFORMATION> {
        let mut process_info = PROCESS_INFORMATION::default();

        let mut command =
            unsafe { WideCString::from_ustr_unchecked(self.calculate_command().as_ustr()) };

        unsafe {
            CreateProcessW(
                None,
                Some(PWSTR::from_raw(command.as_mut_ptr())),
                None,
                None,
                true,
                CREATE_SUSPENDED,
                None,
                None,
                &STARTUPINFOW::default(),
                &mut process_info,
            )?;
        };

        Ok(process_info)
    }

    unsafe fn spawn_shell(&self) -> windows::core::Result<PROCESS_INFORMATION> {
        use windows::Win32::UI::{
            Shell::{SEE_MASK_NOCLOSEPROCESS, SHELLEXECUTEINFOW, ShellExecuteExW},
            WindowsAndMessaging::SW_SHOW,
        };

        let mut process_info = PROCESS_INFORMATION::default();

        let path = unsafe { WideCString::from_ustr_unchecked(self.path.as_ustr()) };
        let args = unsafe { WideCString::from_ustr_unchecked(self.args.as_ustr()) };
        let mut execution_info = SHELLEXECUTEINFOW {
            #[allow(clippy::cast_possible_truncation)]
            cbSize: core::mem::size_of::<SHELLEXECUTEINFOW>() as u32,
            fMask: SEE_MASK_NOCLOSEPROCESS,
            lpFile: PCWSTR::from_raw(path.as_ptr()),
            lpParameters: PCWSTR::from_raw(args.as_ptr()),
            nShow: SW_SHOW.0,
            ..Default::default()
        };

        if let Err(err) = unsafe { ShellExecuteExW(&mut execution_info) } {
            let mut output = String::from("Shim: Unable to create elevated process.\n");
            output.push_str("\t\t- Failed with error: ");
            output.push_str(&err.message());
            output.push('\n');

            error::log_error(output)?;
            error::ExitCode::set_code(1);
            error::ExitCode::set_reason(error::ExitCodeReason::ProcessError);
            error::exit_immediately();
        }

        process_info.hProcess = execution_info.hProcess;

        Ok(process_info)
    }
}
