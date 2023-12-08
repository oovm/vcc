use nyar_error::NyarError;
use std::path::Path;
use wasm_opt::OptimizationOptions;
use wax::{Glob, LinkBehavior};

pub fn optimize_wasm_all(source: &Path, target: &Path, errors: &mut Vec<NyarError>) -> Result<(), NyarError> {
    if !source.exists() {
        std::fs::create_dir_all(source)?;
    }
    if !source.is_dir() {
        return Err(NyarError::custom(format!("Source folder not found: {}", source.display())));
    }
    if !target.exists() {
        std::fs::create_dir_all(target)?;
    }
    if !target.is_dir() {
        return Err(NyarError::custom(format!("Target folder not found: {}", target.display())));
    }
    let glob = Glob::new("*.{wasm}").unwrap();
    for entry in glob.walk_with_behavior(source, LinkBehavior::ReadTarget).not(["**/secret/**"]).unwrap() {
        let file = match entry {
            Ok(o) => o,
            Err(e) => {
                errors.push(NyarError::custom(e));
                continue;
            }
        };
        let path = file.path();
        match optimize_wasm(&path, target) {
            Ok(_) => {}
            Err(e) => errors.push(e),
        }
    }
    Ok(())
}

pub fn optimize_wasm(input: &Path, target: &Path) -> Result<(), NyarError> {
    if cfg!(debug_assertions) {
        if !input.is_file() {
            return Err(NyarError::custom(format!("Input file not found: {}", input.display())));
        }
        if !target.is_dir() {
            return Err(NyarError::custom(format!("Target folder not found: {}", target.display())));
        }
    }
    let output = target.join(input.file_name().unwrap());
    match OptimizationOptions::new_opt_level_4().run(input, output) {
        Ok(_) => Ok(()),
        Err(e) => {
            eprintln!("Optimize: {} failed", input.display());
            Err(NyarError::custom(e.to_string()))
        }
    }
}
