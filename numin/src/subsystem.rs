//! Subsystems for Windows executables

use std::{
    io::{Read, Seek, Write},
    path::PathBuf,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
/// Subsystems for Windows executables
pub enum Subsystem {
    /// Windows GUI subsystem
    Windows,
}

impl Subsystem {
    #[must_use]
    /// Get the subsystem code for the given subsystem.
    pub const fn code(self) -> u16 {
        match self {
            Subsystem::Windows => 2,
        }
    }

    /// Encode the given subsystem into the executable at the given path.
    ///
    /// # Errors
    /// Seeking to particular offsets in the file may fail if the file is not a valid PE executable.
    /// Writing to the file may fail.
    /// See [`std::io::Error`] for more details.
    pub fn encode(self, path: PathBuf) -> std::io::Result<()> {
        let mut file = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .open(path)?;

        file.seek(std::io::SeekFrom::Start(0x3C))?;
        let pe_offset = {
            let mut buf = [0; 4];
            file.read_exact(&mut buf)?;
            u32::from_le_bytes(buf)
        };

        let file_header_offset = file.seek(std::io::SeekFrom::Start(u64::from(pe_offset)))?;

        // Not sure the point of this call
        // reader.seek(std::io::SeekFrom::Current(18))?;

        file.seek(std::io::SeekFrom::Start(file_header_offset + 0x5C))?;

        file.write_all(&self.code().to_le_bytes())?;

        Ok(())
    }
}
