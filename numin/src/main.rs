use std::path::PathBuf;

use clap::Parser;
use numin::{Executable, error, shim::ShimArgs};

#[derive(Debug, Parser)]
struct Args {
    #[clap(help = "Name of the shim")]
    name: String,

    #[clap(help = "Path to the executable to shim")]
    target: PathBuf,

    #[clap(help = "Arguments to pass to the executable from the shim")]
    args: Vec<String>,
}

impl From<Args> for ShimArgs {
    fn from(args: Args) -> Self {
        ShimArgs::new(args.target, args.args)
    }
}

fn main() -> error::Result<()> {
    let args = Args::parse();

    if matches!(PathBuf::from(&args.name).try_exists(), Ok(true)) {
        Err(error::Error::AlreadyExists)?;
    }

    let dest_path = PathBuf::from(&args.name).with_extension("exe");

    let exe = Executable::new();
    let shim = exe.save(&dest_path)?;
    shim.update_resource(args.into())?;

    Ok(())
}
