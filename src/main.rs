#![warn(clippy::all, clippy::pedantic)]
#![no_main]
#![feature(error_generic_member_access)]
#![feature(error_in_core)]
#![feature(try_trait_v2)]

extern crate alloc;

use std::{error::Error, fs::File, io::Read, path::PathBuf};

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

fn _main() -> Result<(), Box<dyn Error>> {
    #[cfg(not(debug_assertions))]
    let filename = {
        let current_exe = std::env::current_exe().unwrap();

        current_exe.with_extension("shim")
    };

    #[cfg(debug_assertions)]
    let filename = PathBuf::from("test.shim");

    let mut file = File::open(&filename)?;

    let skinny_filename = filename.display().to_string();

    let mut shim_string = String::new();

    file.read_to_string(&mut shim_string)?;

    unsafe {
        libc::printf(skinny_filename.as_ptr().cast());
        libc::printf(shim_string.as_ptr().cast());
    }

    Ok(())
}

#[no_mangle]
extern "C" fn main(_argc: isize, _argv: *const *const u8) -> isize {
    _main();

    0
}
