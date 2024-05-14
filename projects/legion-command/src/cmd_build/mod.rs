use clap::Args;

use crate::options::SharedOptions;

#[derive(Debug, Args)]
pub struct RunCommand {
    #[command(flatten)]
    options: SharedOptions,
}

#[derive(Debug, Args)]
pub struct BuildCommand {
    #[command(flatten)]
    options: SharedOptions,
}
#[derive(Debug, Args)]
pub struct TestCommand {
    #[command(flatten)]
    options: SharedOptions,
}
#[derive(Debug, Args)]
pub struct BenchmarkCommand {
    #[command(flatten)]
    options: SharedOptions,
}

#[derive(Debug, Args)]
pub struct PublishCommand {
    #[command(flatten)]
    options: SharedOptions,
}
