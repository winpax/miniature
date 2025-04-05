mod panic;

use alloc::string::String;
use windows::Win32::Foundation::{HANDLE, WIN32_ERROR};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExitCodeReason {
    Unknown,
    ChildError,
    ProcessError,
    Panic,
}

impl ExitCodeReason {
    pub const fn message(&self) -> Option<&str> {
        match self {
            ExitCodeReason::Unknown => Some("Shim: Unknown error\n"),
            ExitCodeReason::ProcessError => Some("Shim: Process error\n"),
            ExitCodeReason::ChildError | ExitCodeReason::Panic => None,
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ExitCode(u32);

impl ExitCode {
    pub const PANIC_EXIT_CODE: u32 = u32::from_le_bytes(*b"pnkd");

    pub fn code() -> u32 {
        unsafe { EXIT_CODE.0 }
    }

    pub fn reason() -> ExitCodeReason {
        unsafe { EXIT_CODE_REASON }
    }

    pub fn set_code(code: u32) {
        unsafe {
            EXIT_CODE.0 = code;
        }
    }

    pub fn set_reason(reason: ExitCodeReason) {
        unsafe {
            EXIT_CODE_REASON = reason;
        }
    }
}

static mut EXIT_CODE_REASON: ExitCodeReason = ExitCodeReason::Unknown;
static mut EXIT_CODE: ExitCode = ExitCode(0);

unsafe fn get_stderr() -> windows::core::Result<HANDLE> {
    unsafe {
        windows::Win32::System::Console::GetStdHandle(
            windows::Win32::System::Console::STD_ERROR_HANDLE,
        )
    }
}

pub fn log_error(message: impl AsRef<str>) -> windows::core::Result<()> {
    let message = message.as_ref();
    let buf = message.as_bytes();

    unsafe {
        windows::Win32::Storage::FileSystem::WriteFile(get_stderr()?, Some(buf), None, None)?;
    }

    Ok(())
}

pub fn exit_immediately() -> ! {
    unsafe { windows::Win32::System::Threading::ExitProcess(ExitCode::code()) }
}

pub fn handle_windows_error(error: WIN32_ERROR) -> ! {
    let mut output = String::from("Shim: An error occurred.\n");
    output.push_str("\t\t- Failed with error: ");
    output.push_str(&error.to_hresult().message());
    output.push('\n');

    let res = log_error(output);
    if let Err(err) = res {
        _ = log_error(err.message());
    }

    exit_immediately();
}
