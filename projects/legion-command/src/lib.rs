use std::ffi::OsString;

use clap::Parser;

pub use crate::{
    cmd_build::BuildCommand,
    options::{ApplicationCommand, SharedOptions},
};

mod cmd_bin_manager;
mod cmd_build;
mod cmd_pkg_manager;
mod options;

#[derive(Parser)]
#[command(version, about, long_about = include_str!("readme.md"))]
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
