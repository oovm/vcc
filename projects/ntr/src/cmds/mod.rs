use clap::{Parser, Subcommand};
use nyar_error::NyarError;

use crate::{BuildCommand, RunCommand};

pub mod build;
pub mod publish;
pub mod run;
pub mod test;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Valor {
    #[clap(subcommand)]
    command: ValorCommands,
}

#[derive(Subcommand)]
pub enum ValorCommands {
    #[clap(name = "build")]
    Build(BuildCommand),
    #[clap(name = "run")]
    Run(RunCommand),
}

impl Valor {
    pub fn run(&self) -> Result<(), NyarError> {
        match &self.command {
            ValorCommands::Build(cmd) => cmd.run(),
            ValorCommands::Run(cmd) => cmd.run(),
        }
    }
}
