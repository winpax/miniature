pub mod shim;

use std::{io::Write, path::Path};

use shim::Shim;

static MINIATURE: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/miniature.exe"));

#[derive(Debug, Copy, Clone)]
pub struct Executable(&'static [u8]);

impl Default for Executable {
    fn default() -> Self {
        Executable::new()
    }
}

impl Executable {
    pub const fn new() -> Self {
        Executable(MINIATURE)
    }

    pub fn save(self, dest_path: impl AsRef<Path>) -> std::io::Result<shim::Shim> {
        let path = dest_path.as_ref().to_owned();
        let mut file = std::fs::File::create(&path)?;
        file.write_all(self.0)?;

        Ok(Shim { path })
    }
}
