// #![deny(missing_debug_implementations, missing_copy_implementations)]
#![warn(missing_docs, rustdoc::missing_crate_level_docs)]
#![doc = include_str!("../readme.md")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]

mod errors;
mod cmds;

pub(crate) mod printer;
pub(crate) mod optimizer;

pub(crate) mod runner;

use clap::{Parser, Subcommand};
pub use crate::errors::{Error, Result};
pub use crate::cmds::build::BuildCommand;
pub use crate::cmds::run::RunCommand;
pub use crate::cmds::{Valor, ValorCommands};