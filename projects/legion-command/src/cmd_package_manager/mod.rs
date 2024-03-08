use std::path::PathBuf;

use clap::Args;

use semver::{Error, VersionReq};
use url::Url;

use crate::options::SharedOptions;

/// Add a
#[derive(Debug, Args)]
pub struct AddCommand {
    /// Reference to a package to add as a dependency
    name_version: PackageVersion,
    /// Filesystem path to local crate to add
    #[arg(long)]
    path: Option<PathBuf>,
    /// Git repository location
    #[arg(long)]
    git: Option<Url>,
    // Space or comma separated list of features to activate
    #[arg(long, short = 'F')]
    features: Vec<String>,
    /// Weather to enable the default features (default = true)
    #[arg(long, default_value_t = true)]
    default_features: bool,
    #[command(flatten)]
    options: SharedOptions,
}

impl AddCommand {
    pub fn run(&self) {
        println!("{self:#?}")
    }
}

#[derive(Debug, Args)]
pub struct UpdateCommand {
    #[command(flatten)]
    options: SharedOptions,
}

impl UpdateCommand {
    pub fn run(&self) {
        println!("{self:#?}")
    }
}

#[derive(Debug, Args)]
pub struct RemoveCommand {
    #[command(flatten)]
    options: SharedOptions,
}

impl RemoveCommand {
    pub(crate) fn run(&self) {
        todo!()
    }
}
