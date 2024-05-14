use crate::{LegionError, LegionErrorKind};
use std::env::VarError;

impl From<LegionErrorKind> for LegionError {
    fn from(error: LegionErrorKind) -> Self {
        Self { kind: error }
    }
}

impl From<std::io::Error> for LegionError {
    fn from(error: std::io::Error) -> Self {
        Self { kind: LegionErrorKind::IoError { source: error, path: None } }
    }
}

impl From<VarError> for LegionError {
    fn from(error: VarError) -> Self {
        Self { kind: LegionErrorKind::CustomError { message: error.to_string() } }
    }
}
