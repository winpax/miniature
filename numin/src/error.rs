#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Failed to create the executable: {0}")]
    Windows(#[from] windows::core::Error),
    #[error("Failed to create the executable: {0}")]
    Io(#[from] std::io::Error),
    #[error("Invalid OsString: {0}")]
    ContainsNul(#[from] widestring::error::ContainsNul<u16>),
    #[error("Executable already exists")]
    AlreadyExists,
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
