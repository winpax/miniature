mod convert;
mod create;

#[derive(Debug, Clone, clap::Subcommand)]
pub enum Commands {
    /// Create a new miniature shim
    Create(create::Args),
    /// Convert existing shims to the miniature format
    Convert(convert::Args),
}
