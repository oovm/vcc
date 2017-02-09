use clap::Parser;
use vcc::{Valor};

fn main() {
    let opts: Valor = Valor::parse();
    opts.run()
}