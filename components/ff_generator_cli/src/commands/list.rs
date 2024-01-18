use std::path::PathBuf;

use structopt::StructOpt;
use anyhow::Result;

#[derive(Debug, StructOpt)]
pub struct ListCommand {}

pub fn list_config(config_file: PathBuf) -> Result<()> {
println!("Querying project {:?}", config_file);
Ok(())
}

