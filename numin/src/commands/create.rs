use std::path::PathBuf;

use numin::{Executable, shim::ShimArgs};

use crate::error;

#[derive(Debug, Clone, clap::Args)]
pub struct Args {
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

    #[clap(from_global)]
    no_subsystem: bool,
}

impl From<Args> for ShimArgs {
    fn from(args: Args) -> Self {
        ShimArgs::new(args.target, args.shim_args)
    }
}

impl Args {
    pub fn run(&self) -> error::Result<()> {
        let dest_path = PathBuf::from(&self.name).with_extension("exe");

        if matches!(dest_path.try_exists(), Ok(true)) {
            Err(error::Error::AlreadyExists)?;
        }

        Executable::create_and_update(self.clone().into(), dest_path, self.no_subsystem)?;

        Ok(())
    }
}
