use nyar_error::NyarError;
use std::{fs::File, io::Write, path::Path};
use wasmprinter::Printer;
use wax::{Glob, LinkBehavior};

pub fn print_wat_all(folder: &Path) -> Result<(), NyarError> {
    let glob = Glob::new("*.{wasm}").unwrap();
    for entry in glob.walk_with_behavior(folder, LinkBehavior::ReadTarget).not(["**/secret/**"]).unwrap() {
        let file = match entry {
            Ok(o) => o,
            Err(e) => Err(NyarError::custom(e))?,
        };
        let path = file.path();
        match print_wat(&path)? {
            Some(s) => {
                let mut file = File::create(path.with_extension("wat"))?;
                file.write_all(s.as_bytes())?;
            }
            None => continue,
        }
    }
    Ok(())
}

pub fn print_wat(file: &Path) -> Result<Option<String>, NyarError> {
    let bytes = std::fs::read(&file)?;
    // println!("Find file: {}", path.display());
    let mut printer = Printer::new();
    printer.print_offsets(false);
    printer.print_skeleton(false);
    match printer.print(&bytes) {
        Ok(o) => Ok(Some(o)),
        Err(e) => {
            eprintln!("Print: {} failed", file.display());
            Err(NyarError::custom(e.to_string()))
        }
    }
}
