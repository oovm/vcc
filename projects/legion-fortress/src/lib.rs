use legion_error::{LegionError, Result};
use semver::Version;
use std::{
    fmt::{Display, Formatter},
    fs::create_dir_all,
    io::ErrorKind,
    path::{Path, PathBuf},
};
use winreg::{enums::HKEY_CURRENT_USER, RegKey};

const LEGION_FORTRESS: &str = r#"LEGION_FORTRESS"#;

pub struct LegionFortress {
    root: PathBuf,
    runner: RunnerID,
}

pub struct PackageInstall {
    pub names: Vec<String>,
    pub version: Version,
    pub alias: String,
}

pub struct RunnerID {
    pub runner: String,
    pub version: Version,
}
impl Display for RunnerID {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}@{}", self.runner, self.version)
    }
}
impl Display for PackageInstall {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}@{}", self.names.join("꞉"), self.version)
    }
}

impl PackageInstall {
    pub fn name(&self) -> &str {
        if !self.alias.is_empty() {
            return &self.alias;
        }
        self.names.last().expect("PackageID must have at least one name")
    }
}

impl LegionFortress {
    pub fn install_script(&self, package: PackageInstall) -> Result<()> {
        let cmd = format!(
            "@echo off

set RUNNER_PATH=\"{runner}.exe\"
set TARGET_PATH=\"{target}.wasm\"

%RUNNER_PATH% %TARGET_PATH% %*

::: if %errorlevel% neq 0 (
:::     echo Error occurred while running the WASM program.
:::     exit /b %errorlevel%
::: )
",
            runner = self.executable_path(self.runner.to_string()).display(),
            target = self.executable_path(package.to_string()).display(),
        );
        let path = self.executable_path(format!("{}.cmd", package.name()));
        match std::fs::write(&path, cmd) {
            Ok(o) => Ok(o),
            Err(e) => LegionError::io_error(e, path),
        }
    }
    pub fn executable_path<P: AsRef<Path>>(&self, path: P) -> PathBuf {
        self.root.join("executable").join(path)
    }
}

impl LegionFortress {
    pub fn new() -> Result<LegionFortress> {
        let dir = match get_home_from_path().or(get_home_from_fs()) {
            Some(s) => s,
            None => {
                return LegionError::io_message(
                    ErrorKind::NotFound,
                    "`legion` home not found".to_string(),
                    PathBuf::from(LEGION_FORTRESS),
                );
            }
        };
        if dir.exists() {
            if !dir.is_dir() {
                return LegionError::io_message(
                    ErrorKind::InvalidInput,
                    format!("`{}` is not a directory", dir.display()),
                    dir,
                );
            }
        }
        else {
            create_dir_all(&dir)?
        }
        match dir.canonicalize() {
            Ok(o) => Ok(Self {
                root: o,
                runner: RunnerID {
                    runner: "wasmtime".to_string(),
                    version: Version { major: 20, minor: 0, patch: 2, pre: Default::default(), build: Default::default() },
                },
            }),
            Err(e) => LegionError::io_error(e, dir),
        }
    }
    /// Register the executable directory to path
    #[cfg(unix)]
    pub fn insert_path(&self) -> Result<()> {
        let dir = self.root.join("executable");
        let path = LegionError::get_env("PATH")?;
        for path in path.split(':') {
            if dir.eq(path) {
                return Ok(());
            }
        }
        let append = format!("{}:{}", dir.display(), path);
        std::env::set_var("PATH", append);
        Ok(())
    }

    /// Register the executable directory to path
    #[cfg(windows)]
    pub fn insert_path(&self) -> Result<()> {
        let dir = self.root.join("executable");
        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        let (env, _) = hkcu.create_subkey("Environment")?;
        let path: String = env.get_value("PATH")?;
        for path in path.split(';') {
            if dir.as_os_str().eq(path) {
                return Ok(());
            }
        }
        let append = format!("{};{}", dir.display(), path);
        env.set_value("PATH", &append)?;
        Ok(())
    }
    /// Remove the executable directory from path
    pub fn delete_path(&self) -> Result<()> {
        let path = LegionError::get_env("PATH")?;
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

fn get_home_from_path() -> Option<PathBuf> {
    match std::env::var(LEGION_FORTRESS) {
        Ok(dir) => {
            if dir.is_empty() {
                println!("`{}` directory is empty", dir);
                None
            }
            else {
                Some(PathBuf::from(dir))
            }
        }
        Err(e) => {
            tracing::trace!("Error reading LEGION_FORTRESS environment variable: {}", e);
            None
        }
    }
}

fn get_home_from_fs() -> Option<PathBuf> {
    match dirs::home_dir() {
        Some(home_dir) => Some(home_dir.join(".legion")),
        None => {
            tracing::trace!("Failed to find the home directory");
            None
        }
    }
}
