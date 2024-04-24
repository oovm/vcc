use std::{
    error::Error,
    fmt::{Debug, Display, Formatter},
};

pub type Result<T, E = LegionConfigError> = std::result::Result<T, E>;

#[derive(Debug)]
pub enum LegionConfigError {
    NotInheritable { field: &'static str },
    MissingField { field: &'static str },
}

impl Display for LegionConfigError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotInheritable { field } => {
                write!(f, "`{}` is not inheritable", field)
            }
            Self::MissingField { field } => {
                write!(f, "`{}` is missing", field)
            }
        }
    }
}

impl Error for LegionConfigError {}
