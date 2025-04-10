use std::path::PathBuf;

use widestring::WideCString;
use windows::{
    Win32::{
        Media::KernelStreaming::RT_STRING,
        System::{
            LibraryLoader::{BeginUpdateResourceW, EndUpdateResourceW, UpdateResourceW},
            SystemServices::{LANG_NEUTRAL, SUBLANG_NEUTRAL},
        },
    },
    core::PCWSTR,
};

use crate::{
    error,
    interop::{MAKEINTRESOURCE, MAKELANGID},
    table,
};

#[derive(Debug, Clone)]
pub struct ShimArgs {
    target: PathBuf,
    args: Vec<String>,
}

impl ShimArgs {
    pub fn new(target: PathBuf, args: Vec<String>) -> Self {
        Self { target, args }
    }
}

#[derive(Debug, Clone)]
pub struct Shim {
    pub(crate) path: PathBuf,
}

impl Shim {
    pub fn update_resource(self, args: ShimArgs) -> error::Result<()> {
        let c_path = WideCString::from_os_str(self.path.as_os_str())?.into_boxed_ucstr();

        let Ok(exe_handle) =
            (unsafe { BeginUpdateResourceW(PCWSTR::from_raw(c_path.as_ptr()), true) })
        else {
            let error = windows::core::Error::from_win32();
            let error = error.message();
            println!("Failed to create the executable: {error}");
            return Ok(());
        };

        let data = {
            let c_exe = WideCString::from_os_str(args.target.as_os_str())?.into_boxed_ucstr();
            let c_args = WideCString::from_str(args.args.join(" "))?.into_boxed_ucstr();

            let mut data = table::StringTable::default();
            data.set_path(Box::leak(c_exe));
            data.set_args(Box::leak(c_args));

            data
        };

        let mut table_buffer = Vec::<u16>::new();

        for entry in data.iter() {
            let entry = *entry;
            table_buffer.push((entry.len() + 1) as u16);

            let entry_buffer = entry.as_slice_with_nul();
            table_buffer.extend(entry_buffer);
        }

        let table_size = table_buffer.len() * std::mem::size_of::<u16>();

        unsafe {
            UpdateResourceW(
                exe_handle,
                RT_STRING,
                MAKEINTRESOURCE!(1),
                MAKELANGID(LANG_NEUTRAL, SUBLANG_NEUTRAL),
                Some(table_buffer.as_ptr().cast()),
                table_size as u32,
            )?
        };

        unsafe { EndUpdateResourceW(exe_handle, false)? };
        Ok(())
    }
}
