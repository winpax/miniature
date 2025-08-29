pub mod shim;

use std::{io::Write, path::Path};

use common::exe_type::ExeType;
use shim::Shim;
use widestring::WideCString;

use crate::error;

static MINIATURE: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/miniature.exe"));

#[derive(Debug, Copy, Clone)]
/// Shim executable data
pub struct Executable(&'static [u8]);

impl Default for Executable {
    fn default() -> Self {
        Executable::DEFAULT
    }
}

impl Executable {
    /// Default [`Executable`] instance with the default binary data.
    pub const DEFAULT: Executable = Executable::new(MINIATURE);

    #[must_use]
    /// Constructs a custom [`Executable`] instance with the given binary data.
    pub const fn new(binary: &'static [u8]) -> Self {
        Executable(binary)
    }

    /// Creates a new [`Executable`] instance with the given data.
    ///
    /// # Errors
    /// IO error. See [`std::io::Error`].
    pub fn save(self, dest_path: impl AsRef<Path>) -> error::Result<shim::Shim> {
        let path = dest_path.as_ref().to_owned();
        let mut file = std::fs::File::create(&path)?;
        file.write_all(self.0)?;

        Ok(Shim { path })
    }

    pub fn create_and_update(
        args: shim::ShimArgs,
        dest_path: impl AsRef<Path>,
        no_subsystem: bool,
    ) -> error::Result<()> {
        let exe = Executable::default();
        let shim = exe.save(&dest_path)?;
        shim.set_resource(args.clone())?;

        let wide_path = WideCString::from_os_str(args.target.as_os_str())?;
        let exe_type = ExeType::from_path(&wide_path).expect("Failed to get the executable type");

        if exe_type.is_windows() {
            println!("Target executable is a GUI application");
            if no_subsystem {
                println!("NOT setting the subsystem to Windows GUI, as requested");
            } else {
                println!("Setting the subsystem to Windows GUI");
                let subsystem = crate::subsystem::Subsystem::Windows;
                subsystem.encode(dest_path.as_ref().to_owned())?;
            }
        }

        Ok(())
    }
}
