pub mod list;
pub mod aws_deploy;

use std::path::PathBuf;
use anyhow::Result;
use structopt::StructOpt;

use self::list::list_config;

#[derive(Debug, StructOpt)]
pub enum Command {
    #[structopt(name = "ls", about = "Lists existing components")]
    List,
    #[structopt(name = "aws", about = "Generates a project for deploying resources to AWS")]
    AwsDeploy
}

impl Command {
    pub fn execute(&self, config: PathBuf) -> Result<()> {
        match *self {
            Command::List => {
                list_config(config)
            }
            Command::AwsDeploy => {
                Ok(())
            },
        }

    }
}