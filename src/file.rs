use core::ptr;

use alloc::{string::String, vec::Vec};
use widestring::WideCStr;
use windows::{
    core::HSTRING,
    Win32::{
        Foundation::{CloseHandle, GENERIC_READ},
        Storage::FileSystem::{
            CreateFileW, ReadFile, ACCESS_DELETE, ACCESS_READ, FILE_ATTRIBUTE_NORMAL,
            FILE_SHARE_READ, OPEN_EXISTING,
        },
        System::LibraryLoader::GetModuleFileNameW,
    },
};

pub struct File {
    handle: windows::Win32::Foundation::HANDLE,
}

impl File {
    pub unsafe fn open(path: &WideCStr) -> windows::core::Result<Self> {
        let path = windows::core::PCWSTR::from_raw(path.as_ptr());

        let handle = CreateFileW(
            path,
            GENERIC_READ.0,
            FILE_SHARE_READ,
            None,
            OPEN_EXISTING,
            FILE_ATTRIBUTE_NORMAL,
            None,
        )?;

        Ok(Self { handle })
    }

    pub fn read(&self, buffer: &mut [u8]) -> windows::core::Result<usize> {
        let mut bytes_read: u32 = 0;

        unsafe {
            ReadFile(
                self.handle,
                Some(buffer),
                Some(ptr::from_mut::<u32>(&mut bytes_read)),
                None,
            )?;
        }

        Ok(bytes_read as usize)
    }

    pub fn read_all(&self) -> windows::core::Result<Vec<u8>> {
        let mut buffer: Vec<u8> = Vec::new();

        loop {
            let bytes_read = self.read(&mut buffer)?;
            if bytes_read == 0 {
                break;
            }
            buffer.resize(bytes_read + buffer.len(), 0);
        }

        Ok(buffer)
    }

    pub fn read_to_string(&self) -> windows::core::Result<String> {
        let buffer = self.read_all()?;

        Ok(String::from_utf8(buffer)?)
    }

    pub unsafe fn close(&self) -> windows::core::Result<()> {
        CloseHandle(self.handle)?;
        Ok(())
    }
}

impl Drop for File {
    fn drop(&mut self) {
        unsafe { self.close().unwrap() };
    }
}
