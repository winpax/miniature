mod spawn;

use alloc::string::String;
use spawn::Spawn;
use windows::Win32::{
    Foundation::{ERROR_ELEVATION_REQUIRED, HANDLE},
    System::{Console::SetConsoleCtrlHandler, Threading::PROCESS_INFORMATION},
};

use crate::{error, resource::ChildResource};

pub struct Job(HANDLE);

impl Job {
    pub unsafe fn new() -> windows::core::Result<Self> {
        unsafe {
            use windows::Win32::System::JobObjects;

            let job_handle = JobObjects::CreateJobObjectW(None, None)?;

            let mut job_info = JobObjects::JOBOBJECT_EXTENDED_LIMIT_INFORMATION::default();
            job_info.BasicLimitInformation.LimitFlags =
                JobObjects::JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE
                    | JobObjects::JOB_OBJECT_LIMIT_SILENT_BREAKAWAY_OK;

            #[allow(clippy::cast_possible_truncation)]
            JobObjects::SetInformationJobObject(
                job_handle,
                JobObjects::JobObjectExtendedLimitInformation,
                core::ptr::from_ref(&job_info).cast(),
                core::mem::size_of_val(&job_info) as u32,
            )?;

            Ok(Self(job_handle))
        }
    }

    pub unsafe fn start(self, resource: &ChildResource) -> windows::core::Result<RunningJob> {
        unsafe {
            use windows::Win32::System::{
                JobObjects::AssignProcessToJobObject, Threading::ResumeThread,
            };

            let command = resource.calculate_command();

            let mut process_info = PROCESS_INFORMATION::default();

            if let Err(err) = resource.spawn_command() {
                if err.code() == ERROR_ELEVATION_REQUIRED.to_hresult() {
                    process_info = resource.spawn_shell()?;
                } else {
                    let mut output = String::from("Shim: Could not create process with command ");
                    output.push_str(&command.to_string_lossy());
                    output.push('.');
                    output.push('\n');
                    output.push_str("\t\t- Failed with error: ");
                    output.push_str(&err.message());
                    output.push('\n');

                    error::log_error(output)?;
                    error::set_exit_code(1);
                    error::exit_immediately();
                }
            } else {
                AssignProcessToJobObject(self.0, process_info.hProcess)?;
                // Cast occurs here because ResumeThread returns a DWORD, but errors return -1.
                #[allow(clippy::cast_possible_wrap)]
                let res = ResumeThread(process_info.hThread) as i32;

                if res < 0 {
                    error::handle_windows_error();
                }
            }

            if SetConsoleCtrlHandler(Some(super::ctrl_handler), true).is_err() {
                error::log_error(String::from(
                    "Could not set control handler; Ctrl-C behaviour may be invalid.\n",
                ))?;
            }

            Ok(RunningJob {
                handle: self,
                proc_info: process_info,
            })
        }
    }
}

pub struct RunningJob {
    handle: Job,
    proc_info: PROCESS_INFORMATION,
}

impl RunningJob {
    pub unsafe fn wait(&self) -> windows::core::Result<u32> {
        unsafe {
            use windows::Win32::{
                Foundation::CloseHandle,
                System::Threading::{GetExitCodeProcess, INFINITE, WaitForSingleObject},
            };

            WaitForSingleObject(self.proc_info.hProcess, INFINITE);

            let mut exit_code = 0u32;
            GetExitCodeProcess(self.proc_info.hProcess, &mut exit_code)?;

            CloseHandle(self.proc_info.hThread)?;
            CloseHandle(self.proc_info.hProcess)?;
            CloseHandle(self.handle.0)?;

            Ok(exit_code)
        }
    }
}
