use alloc::string::String;
use widestring::WideCString;
use windows::{
    Win32::{
        Foundation::HANDLE,
        System::Threading::{
            CREATE_SUSPENDED, CreateProcessW, GetStartupInfoW, PROCESS_INFORMATION, ResumeThread,
            STARTUPINFOW,
        },
    },
    core::{PCWSTR, PWSTR},
};

use crate::{error, resource::ChildResource};

pub struct SpawnedChild {
    process_handle: HANDLE,
    thread_handle: Option<HANDLE>,
}

impl SpawnedChild {
    pub fn process_handle(&self) -> HANDLE {
        self.process_handle
    }

    pub fn thread_handle(&self) -> Option<HANDLE> {
        self.thread_handle
    }
}

pub trait Spawn {
    unsafe fn spawn_command(&self) -> windows::core::Result<SpawnedChild>;
    unsafe fn spawn_shell(&self) -> windows::core::Result<SpawnedChild>;
}

impl Spawn for ChildResource {
    unsafe fn spawn_command(&self) -> windows::core::Result<SpawnedChild> {
        let mut process_info = PROCESS_INFORMATION::default();
        let startup_information = {
            let mut info = STARTUPINFOW::default();
            unsafe { GetStartupInfoW(&mut info) };
            info
        };

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
                &startup_information,
                &mut process_info,
            )?;
        };

        unsafe { ResumeThread(process_info.hThread) };

        Ok(SpawnedChild {
            process_handle: process_info.hProcess,
            thread_handle: Some(process_info.hThread),
        })
    }

    unsafe fn spawn_shell(&self) -> windows::core::Result<SpawnedChild> {
        use windows::Win32::UI::{
            Shell::{SEE_MASK_NOCLOSEPROCESS, SHELLEXECUTEINFOW, ShellExecuteExW},
            WindowsAndMessaging::SW_SHOW,
        };

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
            error::ExitCode::set_reason(error::ExitCode::ProcessError);
            error::exit_immediately();
        }

        Ok(SpawnedChild {
            process_handle: execution_info.hProcess,
            thread_handle: None,
        })
    }
}
