use core::ffi::c_int;

use alloc::vec::Vec;
use widestring::{U16CString, WideChar, WideString};

pub struct Args {
    argc: c_int,
    args: &'static [*const WideChar],
}

impl Args {
    pub const fn null() -> Self {
        Self { argc: 0, args: &[] }
    }

    #[allow(clippy::similar_names)]
    pub fn new(argc: c_int, argv: *const *const WideChar) -> Self {
        let args = unsafe { core::slice::from_raw_parts(argv, argc as usize) };
        Self { argc, args }
    }

    pub fn args(&self) -> Vec<WideString> {
        let mut arg_iter = self
            .args
            .iter()
            .map(|&arg| unsafe { U16CString::from_ptr_truncate(arg, 255) }.to_ustring());

        arg_iter.next();

        arg_iter.collect()
    }
}
