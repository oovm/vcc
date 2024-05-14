use crate::{LegionError, LegionErrorKind};

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
