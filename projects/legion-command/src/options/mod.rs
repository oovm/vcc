use std::io::ErrorKind;

use clap::{Args, Subcommand, ValueEnum};

use crate::{
    BuildCommand,
    cmd_package_manager::{AddCommand, RemoveCommand, UpdateCommand},
};

/// Doc comment
#[derive(Subcommand)]
pub enum ApplicationCommand {
    Add(AddCommand),
    Update(UpdateCommand),
    #[command(alias = "rm")]
    Remove(RemoveCommand),
    Build(BuildCommand),
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
            ApplicationCommand::Update(v) => v.run(),
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
