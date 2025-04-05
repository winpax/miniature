use std::io::Write;

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

    pub fn save(self) -> std::io::Result<()> {
        // let mut file = std::fs::File::create("miniature.exe")?;
        file.write_all(self.0)?;
        Ok(())
    }
}
