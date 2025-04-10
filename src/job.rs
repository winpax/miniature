mod spawn;

use alloc::string::String;
use spawn::Spawn;
use windows::Win32::{
    Foundation::{ERROR_ELEVATION_REQUIRED, GetLastError, HANDLE},
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

            // TODO: Reset to spawn_command
            let process_info = match resource.spawn_command() {
                Err(err) => {
                    if err.code() == ERROR_ELEVATION_REQUIRED.to_hresult() {
                        resource.spawn_shell()?
                    } else {
                        let mut output =
                            String::from("Shim: Could not create process with command ");
                        output.push_str(&resource.calculate_command().to_string_lossy());
                        output.push('.');
                        output.push('\n');
                        output.push_str("\t\t- Failed with error: ");
                        output.push_str(&err.message());
                        output.push('\n');

                        error::log_error(output)?;
                        error::ExitCode::set_code(1);
                        error::ExitCode::set_reason(error::ExitCodeReason::ProcessError);
                        error::exit_immediately();
                    }
                }
                Ok(process_info) => {
                    AssignProcessToJobObject(self.0, process_info.hProcess)?;
                    // Cast occurs here because ResumeThread returns a DWORD, but errors return -1.
                    #[allow(clippy::cast_possible_wrap)]
                    let res = ResumeThread(process_info.hThread) as i32;

                    if res < 0 {
                        let last_error = GetLastError();
                        if last_error.0 != 0 {
                            error::handle_windows_error(last_error);
                        } else {
                            let output =
                                String::from("Shim: Resuming child failed with unknown error.\n");

                            error::log_error(output)?;
                            error::ExitCode::set_code(1);
                            error::ExitCode::set_reason(error::ExitCodeReason::Unknown);
                            error::exit_immediately();
                        }
                    }

                    process_info
                }
            };

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

            // If spawned with shell, the thread handle is invalid.
            if !self.proc_info.hThread.is_invalid() {
                CloseHandle(self.proc_info.hThread)?;
            }
            CloseHandle(self.proc_info.hProcess)?;
            CloseHandle(self.handle.0)?;

            Ok(exit_code)
        }
    }
}
