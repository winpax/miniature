//! Handles updating a shim executable that is saved locally on disk

use std::path::{Path, PathBuf};

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
    table::StringTable,
};

#[derive(Debug, Clone)]
/// Arguments to be passed to the shim executable.
pub struct ShimArgs {
    pub(crate) target: PathBuf,
    pub(crate) args: Vec<String>,
}

impl ShimArgs {
    #[must_use]
    /// Creates a new [`ShimArgs`] instance with the given target and arguments.
    pub fn new(target: impl Into<PathBuf>, args: Vec<String>) -> Self {
        Self {
            target: target.into(),
            args,
        }
    }
}

#[derive(Debug, Clone)]
/// Locally saved shim executable ready to be updated with the given arguments.
pub struct Shim {
    pub(crate) path: PathBuf,
}

impl Shim {
    #[must_use]
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Update the string table section of the executable with the given arguments.
    ///
    /// # Errors
    /// Converting the path to a wide c string may fail if it contains nul terminators within the string.
    /// See [`widestring::error::ContainsNul`] for more details.
    ///
    /// Internal Windows errors also may occur, see [`windows::core::Error`] for more details.
    pub fn set_resource(&self, args: ShimArgs) -> error::Result<()> {
        let c_path = WideCString::from_os_str(self.path.as_os_str())?.into_boxed_ucstr();

        let Ok(exe_handle) =
            (unsafe { BeginUpdateResourceW(PCWSTR::from_raw(c_path.as_ptr()), true) })
        else {
            let error = windows::core::Error::from_win32();
            let error = error.message();
            println!("Failed to create the executable: {error}");
            return Ok(());
        };

        let table = StringTable::try_from(args)?;
        let table_buffer = table.get_buffer();

        unsafe {
            #[allow(clippy::cast_possible_truncation)]
            UpdateResourceW(
                exe_handle,
                RT_STRING,
                const { MAKEINTRESOURCE!(1) },
                const { MAKELANGID(LANG_NEUTRAL, SUBLANG_NEUTRAL) },
                Some(table_buffer.as_ptr().cast()),
                (table_buffer.len() * std::mem::size_of::<u16>()) as u32,
            )?;
        };

        unsafe { EndUpdateResourceW(exe_handle, false)? };
        Ok(())
    }
}
