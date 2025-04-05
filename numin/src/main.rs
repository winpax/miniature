mod interop;
mod miniature;
mod table;

use std::{io::Write, path::PathBuf};

use clap::Parser;
use interop::{MAKEINTRESOURCE, MAKELANGID};
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

#[derive(Debug, Parser)]
struct Args {
    #[clap(help = "Name of the shim")]
    name: PathBuf,
    #[clap(help = "Arguments to pass to the executable from the shim")]
    args: Vec<String>,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    if matches!(args.name.try_exists(), Ok(true)) {
        anyhow::bail!("Path already exists, cannot create a new shim!");
    }

    let exe = miniature::Executable::new();
    exe.save()?;

    let c_path = WideCString::from_os_str(args.name.as_os_str())?.into_boxed_ucstr();

    let exe_handle = unsafe { BeginUpdateResourceW(PCWSTR::from_raw(c_path.as_ptr()), true)? };

    let data = {
        let mut data = table::StringTable::default();
        data.set_path(Box::leak(c_path));
        data.set_args(Box::leak(
            WideCString::from_os_str(args.name.as_os_str())?.into_boxed_ucstr(),
        ));

        data
    };

    type Word = u16;

    let mut table_buffer = Vec::<Word>::new();

    for entry in data.iter() {
        let entry = *entry;
        table_buffer.push(entry.len() as Word);

        let entry_buffer = entry.as_slice_with_nul();
        table_buffer.extend(entry_buffer);
    }

    let table_size = table_buffer.len() * std::mem::size_of::<Word>();

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
