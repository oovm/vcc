use std::fs::File;
use std::path::Path;
use nyar_error::NyarError;
use wasmprinter::Printer;
use wax::{Glob, LinkBehavior};

pub fn print_wat_all(dir: &Path) -> Result<(), NyarError> {
    let glob = Glob::new("*.{wasm}").unwrap();
    for entry in glob
        .walk_with_behavior("doc", LinkBehavior::ReadTarget)
        .not(["**/secret/**"])
        .unwrap()
    {
        let file = entry?;
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

pub fn print_wat(path: &Path) -> Result<Option<String>, NyarError> {
    let bytes = match path.extension() {
        Some(s) if path.is_file() && s.eq("wasm") => std::fs::read(&path)?,
        _ => return Ok(None),
    };
    // println!("Find file: {}", path.display());
    let mut printer = Printer::new();
    printer.print_offsets(false);
    printer.print_skeleton(false);
    match printer.print(&bytes) {
        Ok(o) => Ok(Some(o)),
        Err(e) => Err(NyarError::custom(e.to_string())),
    }
}
