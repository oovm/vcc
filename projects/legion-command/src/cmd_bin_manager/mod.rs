use crate::SharedOptions;
use clap::Args;

/// Install a command line tool
#[derive(Debug, Args)]
pub struct InstallCommand {
    #[command(flatten)]
    options: SharedOptions,
}
#[derive(Debug, Args)]
pub struct AscendCommand {}

/// Update all command line tools
#[derive(Debug, Args)]
pub struct UpgradeCommand {
    #[command(flatten)]
    options: SharedOptions,
}

impl InstallCommand {
    pub fn run(&self) {
        println!("{self:#?}")
    }
}
