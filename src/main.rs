#![warn(clippy::all, clippy::pedantic)]
#![no_std]
#![no_main]
#![feature(error_generic_member_access)]
#![feature(error_in_core)]
#![feature(try_trait_v2)]

pub mod file;
pub mod wide;

extern crate alloc;

use core::mem::MaybeUninit;

use alloc::borrow::ToOwned;
use file::File;
use libc::wchar_t;
use widestring::{WideCStr, WideCString};
use windows::{
    core::HSTRING,
    Win32::{
        Foundation::{CloseHandle, GENERIC_READ},
        Storage::FileSystem::{
            CreateFileW, ACCESS_DELETE, ACCESS_READ, FILE_ATTRIBUTE_NORMAL, FILE_SHARE_READ,
            OPEN_EXISTING,
        },
        System::LibraryLoader::GetModuleFileNameW,
    },
};

fn ctrl_handler(ctrl_type: u32) -> bool {
    use windows::Win32::System::Console::{
        CTRL_BREAK_EVENT, CTRL_CLOSE_EVENT, CTRL_C_EVENT, CTRL_LOGOFF_EVENT, CTRL_SHUTDOWN_EVENT,
    };

    matches!(
        ctrl_type,
        CTRL_C_EVENT
            | CTRL_CLOSE_EVENT
            | CTRL_LOGOFF_EVENT
            | CTRL_BREAK_EVENT
            | CTRL_SHUTDOWN_EVENT
    )
}

const MAX_PATH: usize = windows::Win32::Foundation::MAX_PATH as usize + 2;

fn _main() {
    let filename = unsafe {
        let mut filename: [wchar_t; MAX_PATH] = {
            let uninit: [MaybeUninit<u16>; MAX_PATH] = MaybeUninit::uninit().assume_init();
            core::mem::transmute(uninit)
        };

        let filename_size = GetModuleFileNameW(None, filename.as_mut()) as usize;

        filename[filename_size - 3] = wchar_t::from(b's');
        filename[filename_size - 2] = wchar_t::from(b'h');
        filename[filename_size - 1] = wchar_t::from(b'i');
        filename[filename_size] = wchar_t::from(b'm');
        filename[filename_size + 1] = wchar_t::from(b'\0');

        WideCStr::from_ptr(filename.as_ptr(), filename_size + 1)
            .unwrap()
            .to_owned()
    };

    #[cfg(debug_assertions)]
    let filename = WideCString::from_str("test.shim").unwrap();

    let file = unsafe { File::open(&filename) }.unwrap();

    let skinny_filename = filename.to_string().unwrap();
    let shim_string = file.read_to_string().unwrap();

    unsafe {
        libc::printf(skinny_filename.as_ptr().cast());
        libc::printf(shim_string.as_ptr().cast());
    }
}

#[no_mangle]
extern "C" fn main(_argc: isize, _argv: *const *const u8) -> isize {
    _main();

    0
}
