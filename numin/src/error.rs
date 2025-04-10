//! Error types for the numin library and binary

#[derive(Debug, thiserror::Error)]
/// Error type for the executable creation process.
pub enum Error {
    /// Internal windows error. See [`windows::core::Error`].
    #[error("Failed to create the executable: {0}")]
    Windows(#[from] windows::core::Error),
    /// IO error. See [`std::io::Error`].
    #[error("Failed to create the executable: {0}")]
    Io(#[from] std::io::Error),
    /// Error converting the path to a wide c string. See [`widestring::error::ContainsNul`].
    #[error("Invalid OsString: {0}")]
    ContainsNul(#[from] widestring::error::ContainsNul<u16>),
    /// Executable path already exists.
    #[error("Executable already exists")]
    AlreadyExists,
}

/// Result type for the executable creation process.
pub type Result<T, E = Error> = std::result::Result<T, E>;
