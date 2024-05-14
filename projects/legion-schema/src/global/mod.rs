use std::{path::PathBuf, ptr::null};

const LEGION_FORTRESS: &str = r#"LEGION_FORTRESS"#;

pub struct LegionFortress {
    root: PathBuf,
}

impl LegionFortress {
    fn new() -> Result<LegionFortress, std::io::Error> {
        let dir = get_home_from_path().or(get_home_from_fs())?;
        if !dir.exists() {
            std::fs::create_dir(dir.clone())?;
            return Ok(Self { root: dir });
        }
        if !dir.is_dir() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("`{}` is not a directory", dir.display()),
            ));
        }
        Ok(Self { root: dir })
    }
}

fn get_home_from_path() -> Option<PathBuf> {
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

fn get_home_from_fs() -> Option<PathBuf> {
    match dirs::home_dir() {
        Some(home_dir) => Some(home_dir.join(".legion")),
        None => {
            println!("Failed to get home directory");
            None
        }
    }
}
