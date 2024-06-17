use crate::LegionFortress;
use legion_error::LegionError;
use semver::Version;
use std::{
    fmt::{Display, Formatter},
    path::{Path, PathBuf},
};

pub struct InstallRequest {
    pub names: Vec<String>,
    pub version: Version,
    pub alias: String,
}

impl Display for InstallRequest {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}@{}.wasm", self.names.join("."), self.version)
    }
}

impl InstallRequest {
    /// Get the name of the WASM file, with `.wasm` extension
    pub fn wasm_name(&self) -> &str {
        if !self.alias.is_empty() {
            return &self.alias;
        }
        self.names.last().expect("PackageID must have at least one name")
    }
}

impl LegionFortress {
    pub fn install_runtime(&self, package: InstallRequest) -> legion_error::Result<()> {
        todo!()
    }

    fn refresh_scripts(&self) -> legion_error::Result<()> {
        todo!()
    }

    pub fn install_wasm(&self, package: InstallRequest) -> legion_error::Result<()> {
        self.create_script(package)
    }
    #[cfg(not(windows))]
    fn create_script(&self, package: InstallRequest) -> legion_error::Result<()> {
        Ok(())
    }
    #[cfg(windows)]
    fn create_script(&self, package: InstallRequest) -> legion_error::Result<()> {
        let cmd = format!(
            "@echo off

set RUNNER_PATH=\"{runner}.exe\"
set TARGET_PATH=\"{target}\"

%RUNNER_PATH% %TARGET_PATH% %*

::: if %errorlevel% neq 0 (
:::     echo Error occurred while running the WASM program.
:::     exit /b %errorlevel%
::: )
",
            runner = self.executable_path(self.runner.to_string()).display(),
            target = self.executable_path(package.to_string()).display(),
        );
        let path = self.executable_path(format!("{}.cmd", package.wasm_name()));
        match std::fs::write(&path, cmd) {
            Ok(o) => Ok(o),
            Err(e) => LegionError::io_error(e, path),
        }
    }
    pub fn executable_path<P: AsRef<Path>>(&self, path: P) -> PathBuf {
        self.root.join("executable").join(path)
    }
}
