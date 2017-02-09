use clap::Parser;
use nyar_error::NyarError;
use vcc::{Valor};

fn main() -> Result<(), NyarError> {
    Valor::parse().run()
}