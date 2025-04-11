use widestring::WideCStr;

use crate::interop::{hiword, loword};

pub struct ExeType {
    file_info: usize,
    hiword: u16,
    loword: u16,
}

impl ExeType {
    pub fn from_path(path: impl AsRef<WideCStr>) -> Self {
        use windows::{
            Win32::{
                Storage::FileSystem::FILE_FLAGS_AND_ATTRIBUTES,
                UI::Shell::{SHFILEINFOW, SHGFI_EXETYPE, SHGetFileInfoW},
            },
            core::PCWSTR,
        };

        let mut file_info = SHFILEINFOW::default();

        let file_info = unsafe {
            SHGetFileInfoW(
                PCWSTR::from_raw(path.as_ref().as_ptr()),
                FILE_FLAGS_AND_ATTRIBUTES::default(),
                Some(&mut file_info),
                core::mem::size_of::<SHFILEINFOW>() as u32,
                SHGFI_EXETYPE,
            )
        };

        Self {
            file_info,
            hiword: unsafe { hiword(file_info) },
            loword: unsafe { loword(file_info) },
        }
    }

    pub fn windows_app(&self) -> bool {
        self.file_info != 0
    }

    #[allow(unused)]
    pub fn is_msdos(&self) -> bool {
        self.loword == MZ
    }

    #[allow(unused)]
    pub fn is_console(&self) -> bool {
        self.loword == PE && self.hiword == 0
    }

    #[allow(unused)]
    pub fn is_windows(&self) -> bool {
        (self.loword == PE || self.loword == NE) && self.hiword != 0
    }
}

const MZ: u16 = u16::from_le_bytes([b'M', b'Z']);
const PE: u16 = u16::from_le_bytes([b'P', b'E']);
const NE: u16 = u16::from_le_bytes([b'N', b'E']);
