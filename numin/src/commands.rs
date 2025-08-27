mod create;

#[derive(Debug, Clone, clap::Subcommand)]
pub enum Commands {
    /// Create a new miniature shim
    Create(create::Args),
}
