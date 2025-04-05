use std::{io::Write, path::Path};

static MINIATURE: &[u8] = include_bytes!("../../miniature.exe");

#[derive(Debug, Copy, Clone)]
pub struct Executable(&'static [u8]);

impl Default for Executable {
    fn default() -> Self {
        Executable(MINIATURE)
    }
}

impl Executable {
    pub const fn new() -> Self {
        Executable(MINIATURE)
    }

    pub fn save(self, dest_path: impl AsRef<Path>) -> std::io::Result<()> {
        let mut file = std::fs::File::create(dest_path)?;
        file.write_all(self.0)?;
        Ok(())
    }
}
