mod spawn;

use alloc::string::String;
use spawn::{Spawn, SpawnedChild};
use windows::Win32::{
    Foundation::{ERROR_ELEVATION_REQUIRED, HANDLE},
    System::Console::SetConsoleCtrlHandler,
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
            use windows::Win32::System::JobObjects::AssignProcessToJobObject;

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
                        error::ExitCode::set_reason(error::ExitCode::ProcessError);
                        error::exit_immediately();
                    }
                }
                Ok(process_info) => {
                    AssignProcessToJobObject(self.0, process_info.process_handle())?;

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
                process_info,
            })
        }
    }
}

pub struct RunningJob {
    handle: Job,
    process_info: SpawnedChild,
}

impl RunningJob {
    pub unsafe fn wait(&self) -> windows::core::Result<u32> {
        unsafe {
            use windows::Win32::{
                Foundation::CloseHandle,
                System::Threading::{GetExitCodeProcess, INFINITE, WaitForSingleObject},
            };

            WaitForSingleObject(self.process_info.process_handle(), INFINITE);

            let mut exit_code = 0u32;
            GetExitCodeProcess(self.process_info.process_handle(), &mut exit_code)?;

            CloseHandle(self.process_info.process_handle())?;
            CloseHandle(self.handle.0)?;

            if let Some(thread_handle) = self.process_info.thread_handle() {
                CloseHandle(thread_handle)?;
            }

            Ok(exit_code)
        }
    }
}
