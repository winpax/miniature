//! Both functions in this file are ported almost directly from the original <https://github.com/71/scoop-better-shimexe/blob/master/shim.c>

use widestring::U16CStr;
use windows::core::PCWSTR;

pub unsafe fn ris_windows_app(path: &U16CStr) -> bool {
    use windows::Win32::{
        Storage::FileSystem::FILE_FLAGS_AND_ATTRIBUTES,
        UI::Shell::{SHGetFileInfoW, SHFILEINFOW, SHGFI_EXETYPE},
    };
    let mut file_info = SHFILEINFOW::default();

    let file_info = unsafe {
        SHGetFileInfoW(
            PCWSTR::from_raw(path.as_ptr()),
            FILE_FLAGS_AND_ATTRIBUTES::default(),
            Some(&mut file_info),
            core::mem::size_of::<SHFILEINFOW>() as u32,
            SHGFI_EXETYPE,
        )
    };

    file_info != 0
}

pub unsafe fn rcompute_program_length(commandline: &[u16]) -> usize {
    let mut i = 0usize;

    if commandline[0] == ('"' as u16) {
        i += 1;
    }

    loop {
        i += 1;
        let char = commandline[i];

        // String already terminated
        if char == 0 {
            i -= 1;
            break;
        }
        // End of string
        if char == ('"' as u16) {
            // Skip the space after the closing quote
            i += 1;
            break;
        }
    }

    i
}
