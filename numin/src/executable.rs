pub mod shim;

use std::{io::Write, path::Path};

use shim::Shim;

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
}
