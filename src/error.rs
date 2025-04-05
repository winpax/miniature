mod panic;

use alloc::string::String;
use windows::Win32::Foundation::{HANDLE, WIN32_ERROR};

pub fn set_exit_code(code: u32) {
    unsafe {
        EXIT_CODE = code;
    }
}

pub fn get_exit_code() -> u32 {
    unsafe { EXIT_CODE }
}

static mut EXIT_CODE: u32 = 0;

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
    unsafe { windows::Win32::System::Threading::ExitProcess(get_exit_code()) }
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
