use clap::Parser;
use nyar_error::NyarError;

use crate::{optimizer::optimize_wasm_all, printer::print_wat_all};

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
        let mut errors = vec![];
        optimize_wasm_all(&source, &target, &mut errors)?;
        print_wat_all(&source)?;
        print_wat_all(&target)?;
        for e in errors {
            eprintln!("{}", e);
        }
        Ok(())
    }
}
