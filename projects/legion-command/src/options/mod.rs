use std::io::ErrorKind;

use clap::{Args, Subcommand, ValueEnum};

use crate::{
    cmd_bin_manager::{AscendCommand, InstallCommand, UpgradeCommand},
    cmd_build::{BenchmarkCommand, PublishCommand, RunCommand, TestCommand},
    cmd_pkg_manager::{AddCommand, GetCommand, RemoveCommand},
    BuildCommand,
};

/// Doc comment
#[derive(Subcommand)]
pub enum ApplicationCommand {
    /// Update legion & vpp to the latest version
    Ascend(AscendCommand),
    /// Install a new wasm binary
    Install(InstallCommand),
    /// Update the wasm binaries
    Upgrade(UpgradeCommand),
    /// Uninstall a wasm binary
    Uninstall(InstallCommand),
    /// Add a new dependency
    Add(AddCommand),
    /// Update the dependencies
    Get(GetCommand),
    /// Remove a dependency
    #[command(alias = "rm")]
    Remove(RemoveCommand),

    /// Build the project
    Run(RunCommand),

    /// Run the tests
    Test(TestCommand),
    /// Run the benchmarks
    #[command(alias = "bench")]
    Benchmark(BenchmarkCommand),
    /// Build the project
    Build(BuildCommand),
    /// Build the project
    Publish(PublishCommand),

    #[command(external_subcommand)]
    External(Vec<String>),
}

#[derive(Debug, Args)]
pub struct SharedOptions {
    #[arg(long)]
    offline: bool,
    #[arg(long)]
    dry_run: bool,
}

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
enum ColorWhen {
    Always,
    Auto,
    Never,
}

impl ApplicationCommand {
    pub fn run(&self) {
        match self {
            ApplicationCommand::Build(v) => {
                println!("build: {v:?}")
            }
            ApplicationCommand::External(v) => match v.as_slice() {
                [cmd, args @ ..] => {
                    self.spawn(cmd, args);
                }
                [] => {}
            },
            ApplicationCommand::Add(v) => v.run(),
            ApplicationCommand::Install(v) => v.run(),
            ApplicationCommand::Remove(v) => v.run(),
        }
    }
    pub fn spawn(&self, command: &String, arguments: &[String]) {
        match std::process::Command::new(format!("vpp-{command}")).args(arguments).spawn() {
            Ok(mut o) => {
                o.wait().unwrap();
            }
            Err(e) => match e.kind() {
                ErrorKind::NotFound => {
                    println!("Command not found: `vpp-{command}`")
                }
                _ => {}
            },
        };
    }
}
