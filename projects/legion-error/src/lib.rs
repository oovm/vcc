use std::{
    error::Error,
    fmt::{Debug, Display, Formatter},
    path::PathBuf,
};

mod from_std;

pub type Result<T = (), E = LegionError> = std::result::Result<T, E>;

pub struct LegionError {
    kind: LegionErrorKind,
}

pub enum LegionErrorKind {
    IoError { source: std::io::Error, path: Option<PathBuf> },
}

impl LegionError {
    pub fn io_error<T>(source: std::io::Error, path: PathBuf) -> Result<T, LegionError> {
        Err(Self { kind: LegionErrorKind::IoError { source, path: Some(path) } })
    }
    pub fn io_message<T>(source: std::io::ErrorKind, message: impl Into<String>, path: PathBuf) -> Result<T, LegionError> {
        Err(Self { kind: LegionErrorKind::IoError { source: std::io::Error::new(source, message), path: Some(path) } })
    }
}

impl Debug for LegionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Display for LegionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Error for LegionError {}

impl Debug for LegionErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Display for LegionErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Error for LegionErrorKind {}
