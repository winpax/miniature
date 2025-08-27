use std::fs::File;

use clap::Parser;
use numin::Executable;
use sprinkles::{
    contexts::{ScoopContext, User},
    progress::{self, Message, ProgressOptions},
};

use crate::error;

#[derive(Debug, Clone, Parser)]
pub struct Args {
    #[clap(from_global)]
    no_subsystem: bool,
}

impl Args {
    pub fn run(&self) -> error::Result<()> {
        println!("WARNING: This command is in BETA. It may break your shims.");

        let ctx = User::new().map_err(|e| sprinkles::contexts::Error::Custom(Box::new(e)))?;

        let shims_path = ctx.shims_path();

        let mut found_shims = vec![];

        for entry in std::fs::read_dir(shims_path)? {
            let entry = entry?;
            let path = entry.path();

            if path.extension() == Some(std::ffi::OsStr::new("shim"))
                && path.with_extension("exe").exists()
            {
                found_shims.push(path);
            }
        }

        let style = progress::style(Some(ProgressOptions::PosLen), Some(Message::prefix()));
        let pb = indicatif::ProgressBar::new(found_shims.len() as u64).with_style(style);

        for shim in found_shims {
            pb.set_message(format!("Converting {}", shim.display()));

            let mut fp = File::create(&shim)?;

            let shim_data = scoop_shim::from_reader(&mut fp)?;
            let dest_path = shim.with_extension("exe");

            std::fs::rename(&dest_path, dest_path.with_extension("old"))?;

            Executable::create_and_update(shim_data.into(), dest_path, self.no_subsystem)?;

            pb.inc(1);
        }

        pb.finish();

        Ok(())
    }
}
