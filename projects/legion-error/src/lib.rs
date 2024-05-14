use std::{
    env::VarError,
    error::Error,
    fmt::{Debug, Display, Formatter},
    path::PathBuf,
};

mod from_std;

pub type Result<T = (), E = LegionError> = std::result::Result<T, E>;

pub struct LegionError {
    kind: LegionErrorKind,
}

#[derive(Debug)]
pub enum LegionErrorKind {
    IoError { source: std::io::Error, path: Option<PathBuf> },
    CustomError { message: String },
}

impl LegionError {
    pub fn io_error<T>(source: std::io::Error, path: PathBuf) -> Result<T, LegionError> {
        Err(Self { kind: LegionErrorKind::IoError { source, path: Some(path) } })
    }
    pub fn get_env(key: &str) -> Result<String, LegionError> {
        match std::env::var(key) {
            Ok(o) => Ok(o),
            Err(VarError::NotPresent) => Self::custom(format!("Environment variable `{}` is not present", key)),
            Err(VarError::NotUnicode(_)) => Self::custom(format!("Environment variable `{}` is not unicode", key)),
        }
    }
    pub fn io_message<T>(source: std::io::ErrorKind, message: impl Into<String>, path: PathBuf) -> Result<T, LegionError> {
        Err(Self { kind: LegionErrorKind::IoError { source: std::io::Error::new(source, message.into()), path: Some(path) } })
    }
    pub fn custom<T>(message: impl Into<String>) -> Result<T, LegionError> {
        Err(Self { kind: LegionErrorKind::CustomError { message: message.into() } })
    }
}

pub fn custom_error(message: impl Into<String>) -> LegionError {
    LegionError { kind: LegionErrorKind::CustomError { message: message.into() } }
}

impl Error for LegionError {}
impl Error for LegionErrorKind {}

impl Debug for LegionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.kind, f)
    }
}

impl Display for LegionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.kind, f)
    }
}

impl Display for LegionErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            LegionErrorKind::IoError { source, path } => match path {
                Some(path) => {
                    write!(f, "{} ({})", source, path.display())
                }
                None => {
                    write!(f, "{}", source)
                }
            },
            LegionErrorKind::CustomError { message } => {
                write!(f, "{}", message)
            }
        }
    }
}
