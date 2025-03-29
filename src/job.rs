use widestring::{U16Str, U16String};
use windows::{
    core::PWSTR,
    Win32::{
        Foundation::{ERROR_ELEVATION_REQUIRED, HANDLE},
        System::{Console::SetConsoleCtrlHandler, Threading::PROCESS_INFORMATION},
    },
};

pub struct Job(HANDLE);

impl Job {
    pub unsafe fn new() -> windows::core::Result<Self> {
        use windows::Win32::System::JobObjects;

        let job_handle = JobObjects::CreateJobObjectW(None, None)?;

        let mut job_info = JobObjects::JOBOBJECT_EXTENDED_LIMIT_INFORMATION::default();
        job_info.BasicLimitInformation.LimitFlags = JobObjects::JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE
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

    pub unsafe fn start(self, command: &U16Str) -> windows::core::Result<RunningJob> {
        use windows::Win32::System::{
            JobObjects::AssignProcessToJobObject,
            Threading::{CreateProcessW, ResumeThread, CREATE_SUSPENDED, STARTUPINFOW},
        };

        let startup_info = STARTUPINFOW::default();
        let mut process_info = PROCESS_INFORMATION::default();

        if let Err(err) = CreateProcessW(
            None,
            Some(PWSTR::from_raw(command.as_ptr().cast_mut())),
            None,
            None,
            true,
            CREATE_SUSPENDED,
            None,
            None,
            &startup_info,
            &mut process_info,
        ) {
            if err.code() == ERROR_ELEVATION_REQUIRED.to_hresult() {
                todo!("Attempt to run as administrator")
            } else {
                let mut output = U16String::from("Shim: Could not create process with command ");
                output.push(command);
                output.push_char('.');
                output.push_char('\n');
                output.push_str("\t\t- Failed with error: ");
                output.push_str(err.message());
                output.push_char('\n');

                super::error::log_error(output)?;
                super::error::set_exit_code(1);
                super::error::exit_immediately();
            }
        } else {
            AssignProcessToJobObject(self.0, process_info.hProcess)?;
            ResumeThread(process_info.hThread);
        }

        if SetConsoleCtrlHandler(Some(super::ctrl_handler), true).is_err() {
            super::error::log_error(U16String::from(
                "Could not set control handler; Ctrl-C behaviour may be invalid.\n",
            ))?;
        }

        Ok(RunningJob {
            handle: self,
            proc_info: process_info,
        })
    }
}

pub struct RunningJob {
    handle: Job,
    proc_info: PROCESS_INFORMATION,
}

impl RunningJob {
    pub unsafe fn wait(&self) -> windows::core::Result<u32> {
        use windows::Win32::{
            Foundation::CloseHandle,
            System::Threading::{GetExitCodeProcess, WaitForSingleObject, INFINITE},
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
