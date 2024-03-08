use std::ffi::OsString;

use clap::Parser;

pub use crate::{
    cmd_build::BuildCommand,
    options::{ApplicationCommand, SharedOptions},
};

mod cmd_build;
mod options;

/// Doc comment
#[derive(Parser)]
pub struct Application {
    /// Doc comment
    #[arg(long, short = 'C')]
    directory: Option<OsString>,
    #[command(subcommand)]
    command: ApplicationCommand,
}

impl Application {
    pub fn run(&self) {
        self.command.run()
    }
}
