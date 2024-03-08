use clap::Args;

use crate::options::SharedOptions;

#[derive(Debug, Args)]
pub struct BuildCommand {
    #[command(flatten)]
    options: SharedOptions,
}
