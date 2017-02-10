// #![deny(missing_debug_implementations, missing_copy_implementations)]
#![warn(missing_docs, rustdoc::missing_crate_level_docs)]
#![doc = include_str!("../readme.md")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]

mod cmds;
mod errors;

pub(crate) mod optimizer;
pub(crate) mod printer;

pub(crate) mod runner;

pub use crate::{
    cmds::{build::BuildCommand, publish::PublishCommand, run::RunCommand, test::TestCommand, Valor, ValorCommands},
    errors::{Error, Result},
};
use clap::{Parser, Subcommand};
