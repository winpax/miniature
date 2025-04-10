#![warn(
    clippy::pedantic,
    clippy::all,
    missing_debug_implementations,
    missing_copy_implementations
)]

use std::path::PathBuf;

use clap::Parser;
use numin::{Executable, error, shim::ShimArgs};

#[derive(Debug, Parser)]
struct Args {
    #[clap(help = "Name of the shim")]
    name: String,

    #[clap(help = "Path to the executable to shim")]
    target: PathBuf,

    #[allow(clippy::struct_field_names)]
    #[clap(
        help = "Arguments to pass to the executable from the shim",
        value_name = "ARGS"
    )]
    shim_args: Vec<String>,
}

impl From<Args> for ShimArgs {
    fn from(args: Args) -> Self {
        ShimArgs::new(args.target, args.shim_args)
    }
}

fn main() -> error::Result<()> {
    let args = Args::parse();

    if matches!(PathBuf::from(&args.name).try_exists(), Ok(true)) {
        Err(error::Error::AlreadyExists)?;
    }

    let dest_path = PathBuf::from(&args.name).with_extension("exe");

    let exe = Executable::default();
    let shim = exe.save(&dest_path)?;
    shim.set_resource(args.into())?;

    Ok(())
}
