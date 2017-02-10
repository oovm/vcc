use clap::Parser;


/// `valor publish`
#[derive(Parser)]
pub struct PublishCommand {
    /// Running under debug mode
    #[clap(long)]
    debug: bool,
    /// Running under release mode
    #[clap(long)]
    release: bool,
    /// Optimize Level
    #[arg(short, long, action = clap::ArgAction::Count)]
    optimize: u8,
}