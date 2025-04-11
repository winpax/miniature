mod panic;

use alloc::string::String;
use windows::Win32::{
    Foundation::{HANDLE, WIN32_ERROR},
    System::Console,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExitCode {
    Unknown(u32),
    ChildError,
    ProcessError,
    Panic,
}

impl ExitCode {
    pub const fn message(self) -> Option<&'static str> {
        match self {
            ExitCode::Unknown(_) => Some("Shim: Unknown error\n"),
            ExitCode::ProcessError => Some("Shim: Process error\n"),
            ExitCode::ChildError | ExitCode::Panic => None,
        }
    }

    pub const fn code(self) -> u32 {
        match self {
            ExitCode::Unknown(code) => code,
            ExitCode::ProcessError => 2,
            ExitCode::ChildError => 3,
            ExitCode::Panic => u32::from_le_bytes(*b"PNCK"),
        }
    }

    pub fn get_code() -> u32 {
        unsafe { EXIT_CODE.code() }
    }

    pub fn reason() -> ExitCode {
        unsafe { EXIT_CODE }
    }

    pub fn set_code(code: u32) {
        unsafe {
            EXIT_CODE = ExitCode::Unknown(code);
        }
    }

    /// This will set the exit code reason to the given value, and set the code to the reason's code
    pub fn set_reason(reason: ExitCode) {
        unsafe {
            EXIT_CODE = reason;
        }
    }
}

static mut EXIT_CODE: ExitCode = ExitCode::Unknown(0);

unsafe fn get_stdout() -> windows::core::Result<HANDLE> {
    unsafe { Console::GetStdHandle(Console::STD_OUTPUT_HANDLE) }
}

unsafe fn get_stderr() -> windows::core::Result<HANDLE> {
    unsafe { Console::GetStdHandle(Console::STD_ERROR_HANDLE) }
}

#[allow(unused)]
pub fn log(message: impl AsRef<str>) -> windows::core::Result<()> {
    let message = message.as_ref();
    let buf = message.as_bytes();

    unsafe {
        Console::WriteConsoleA(get_stdout()?, buf, None, None)?;
    }

    Ok(())
}

pub fn log_error(message: impl AsRef<str>) -> windows::core::Result<()> {
    let message = message.as_ref();
    let buf = message.as_bytes();

    unsafe {
        Console::WriteConsoleA(get_stderr()?, buf, None, None)?;
    }

    Ok(())
}

pub fn exit_immediately() -> ! {
    unsafe { windows::Win32::System::Threading::ExitProcess(ExitCode::get_code()) }
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
