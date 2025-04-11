use widestring::WideCStr;

use crate::interop::{hiword, loword};

pub struct ExeType {
    hiword: u16,
    loword: u16,
}

impl std::fmt::Debug for ExeType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        fn format_bytes(bytes: &[u8]) -> alloc::string::String {
            bytes.iter().map(|byte| char::from(*byte)).collect()
        }

        let formatted_hiword = format_bytes(&self.hiword.to_le_bytes());
        let formatted_loword = format_bytes(&self.loword.to_le_bytes());

        let mut debug_struct = f.debug_struct("ExeType");

        if self.hiword == 0 {
            debug_struct.field("hiword", &None::<String>);
        } else {
            debug_struct.field("hiword", &formatted_hiword);
        }

        if self.loword == 0 {
            debug_struct.field("loword", &None::<String>);
        } else {
            debug_struct.field("loword", &formatted_loword);
        }

        debug_struct.finish()
    }
}

impl ExeType {
    pub fn from_path(path: impl AsRef<WideCStr>) -> Option<Self> {
        use windows::{
            Win32::{
                Storage::FileSystem::FILE_FLAGS_AND_ATTRIBUTES,
                UI::Shell::{SHFILEINFOW, SHGFI_EXETYPE, SHGetFileInfoW},
            },
            core::PCWSTR,
        };

        let mut file_info = SHFILEINFOW::default();

        #[allow(clippy::cast_possible_truncation)]
        let file_info = unsafe {
            SHGetFileInfoW(
                PCWSTR::from_raw(path.as_ref().as_ptr()),
                FILE_FLAGS_AND_ATTRIBUTES::default(),
                Some(&mut file_info),
                core::mem::size_of::<SHFILEINFOW>() as u32,
                SHGFI_EXETYPE,
            )
        };

        if file_info == 0 {
            return None;
        }

        Some(Self {
            hiword: unsafe { hiword(file_info) },
            loword: unsafe { loword(file_info) },
        })
    }

    #[allow(unused)]
    #[must_use]
    pub fn is_msdos(&self) -> bool {
        self.loword == MZ
    }

    #[allow(unused)]
    #[must_use]
    pub fn is_console(&self) -> bool {
        self.loword == PE && self.hiword == 0
    }

    #[allow(unused)]
    #[must_use]
    pub fn is_windows(&self) -> bool {
        (self.loword == PE || self.loword == NE) && self.hiword != 0
    }
}

const MZ: u16 = u16::from_le_bytes([b'M', b'Z']);
const PE: u16 = u16::from_le_bytes([b'P', b'E']);
const NE: u16 = u16::from_le_bytes([b'N', b'E']);
