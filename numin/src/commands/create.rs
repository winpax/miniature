use std::path::PathBuf;

use common::exe_type::ExeType;
use numin::{Executable, shim::ShimArgs};
use widestring::WideCString;

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

    #[clap(help = "Do not set the subsystem of the shim", long, short)]
    no_subsystem: bool,
}

impl From<Args> for ShimArgs {
    fn from(args: Args) -> Self {
        ShimArgs::new(args.target, args.shim_args)
    }
}

impl Args {
    pub fn run(&self) -> error::Result<()> {
        if matches!(PathBuf::from(&self.name).try_exists(), Ok(true)) {
            Err(error::Error::AlreadyExists)?;
        }

        let dest_path = PathBuf::from(&self.name).with_extension("exe");

        let exe = Executable::default();
        let shim = exe.save(&dest_path)?;
        shim.set_resource(self.clone().into())?;

        let wide_path = WideCString::from_os_str(self.target.as_os_str())?;
        let exe_type = ExeType::from_path(&wide_path).expect("Failed to get the executable type");

        if exe_type.is_windows() {
            println!("Target executable is a GUI application");
            if self.no_subsystem {
                println!("NOT setting the subsystem to Windows GUI, as requested");
            } else {
                println!("Setting the subsystem to Windows GUI");
                let subsystem = numin::subsystem::Subsystem::Windows;
                subsystem.encode(dest_path)?;
            }
        }

        Ok(())
    }
}
