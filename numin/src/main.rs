#![warn(
    clippy::pedantic,
    clippy::all,
    missing_debug_implementations,
    missing_copy_implementations
)]

mod commands;

use clap::Parser;
use numin::error;

use crate::commands::Commands;

#[derive(Debug, Clone, Parser)]
struct Args {
    #[clap(subcommand)]
    command: Commands,

    #[clap(
        global = true,
        help = "Do not set the subsystem of the shim",
        long,
        short
    )]
    no_subsystem: bool,
}

fn main() -> error::Result<()> {
    let args = Args::parse();

    match args.command {
        Commands::Create(args) => args.run()?,
        Commands::Convert(args) => args.run()?,
    }

    Ok(())
}
