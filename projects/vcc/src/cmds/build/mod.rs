use clap::Parser;
use nyar_error::NyarError;
use crate::optimizer::optimize_wasm_all;
use crate::printer::print_wat_all;

/// `valor build --release`
#[derive(Parser)]
pub struct BuildCommand {
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

impl BuildCommand {
    pub fn run(&self) -> Result<(), NyarError> {
        let current = std::env::current_dir()?;
        let source = current.join("target/debug/valkyrie");
        let target = current.join("target/release/valkyrie");
        optimize_wasm_all(&source, &target)?;
        print_wat_all(&source)?;
        print_wat_all(&target)?;
        Ok(())
    }
}