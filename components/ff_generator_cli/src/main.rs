//! Doing stuff

mod commands;

use std::path::PathBuf;

use anyhow::Result;
use commands::Command;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt( name = "ff-generator",
    about = "Project generator for a fullstack application")]
struct CLI {
    #[structopt(subcommand)]
    pub command: Command,

    #[structopt(long, short, parse(from_os_str), default_value = "ff_generator_config.toml")]
    pub config: PathBuf,

}
fn main() -> Result<()> {
    let cli = CLI::from_args();

    cli.command.execute(cli.config)
}
