use widestring::U16Str;
use windows::Win32::Foundation::HANDLE;

unsafe fn get_stderr() -> windows::core::Result<HANDLE> {
    windows::Win32::System::Console::GetStdHandle(windows::Win32::System::Console::STD_ERROR_HANDLE)
}

pub fn log_error(message: impl AsRef<U16Str>) -> windows::core::Result<()> {
    let buf = message.as_ref().as_slice();

    unsafe {
        let _ = windows::Win32::System::Console::WriteConsoleW(
            get_stderr()?,
            buf,
            #[allow(clippy::cast_possible_truncation)]
            Some(&mut (buf.len() as u32)),
            None,
        );
    }

    Ok(())
}
