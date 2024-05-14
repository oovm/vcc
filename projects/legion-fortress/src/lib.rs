use legion_error::{LegionError, Result};
use std::{io::ErrorKind, path::PathBuf};
const LEGION_FORTRESS: &str = r#"LEGION_FORTRESS"#;

pub struct LegionFortress {
    root: PathBuf,
}

impl LegionFortress {
    pub fn new() -> Result<LegionFortress> {
        let dir = get_home_from_path().or(get_home_from_fs())?;
        if dir.exists() {
            if !dir.is_dir() {
                return LegionError::io_message(
                    ErrorKind::InvalidInput,
                    format!("`{}` is not a directory", dir.display()),
                    dir,
                );
            }
        }
        match dir.canonicalize() {
            Ok(o) => Ok(Self { root: o }),
            Err(e) => LegionError::io_error(e, dir),
        }
    }
    /// Register the executable directory to path
    pub fn insert_path(&self) -> Result<()> {
        let dir = self.root.join("executable");
        let path = std::env::var("PATH")?;
        let append = if cfg!(os = "windows") {
            for path in path.split(';') {
                if dir.eq(path) {
                    return Ok(());
                }
            }
            format!("{};{}", dir.display(), path)
        }
        else {
            for path in path.split(':') {
                if dir.eq(path) {
                    return Ok(());
                }
            }
            format!("{}:{}", dir.display(), path)
        };
        std::env::set_var("PATH", append);
        Ok(())
    }
    /// Remove the executable directory from path
    pub fn delete_path(&self) -> Result<()> {
        let path = std::env::var("PATH")?;
        let replaced = if cfg!(os = "windows") {
            path.replace(&format!("{};", self.root.display()), "")
        }
        else {
            path.replace(&format!("{}:", self.root.display()), "")
        };
        std::env::set_var("PATH", replaced);
        Ok(())
    }
}

fn get_home_from_path() -> Result<PathBuf> {
    match std::env::var(LEGION_FORTRESS) {
        Ok(dir) => {
            if dir.is_empty() {
                println!("`LEGION_FORTRESS` directory is empty");
                None
            }
            else {
                Some(PathBuf::from(dir))
            }
        }
        Err(e) => {
            println!("Error reading LEGION_FORTRESS environment variable: {}", e);
            None
        }
    }
}

fn get_home_from_fs() -> Result<PathBuf> {
    match dirs::home_dir() {
        Some(home_dir) => Some(home_dir.join(".legion")),
        None => {
            println!("Failed to get home directory");
            None
        }
    }
}
