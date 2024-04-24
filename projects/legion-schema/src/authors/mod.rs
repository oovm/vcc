use crate::{InheritConfig, LegionConfigError, LegionPackageSetting, Result};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, JsonSchema)]
#[schemars(untagged)]
pub enum AuthorsInfo {
    Inherit(InheritConfig),
    Single(String),
    Multiple(Vec<String>),
}

impl Default for AuthorsInfo {
    fn default() -> Self {
        Self::Multiple(vec![])
    }
}

impl AuthorsInfo {
    /// Seek the authors from package and workspace
    pub fn seek(&self, ws: &Option<LegionPackageSetting>) -> Result<Vec<String>> {
        match self {
            Self::Inherit(InheritConfig::Workspace) => match ws {
                Some(ws) => match &ws.authors {
                    Self::Inherit(_) => Err(LegionConfigError::NotInheritable { field: "workspace.authors" }),
                    Self::Single(s) => Ok(vec![s.to_owned()]),
                    Self::Multiple(s) => Ok(s.to_owned()),
                },
                None => Ok(vec![]),
            },
            Self::Single(s) => Ok(vec![s.to_owned()]),
            Self::Multiple(s) => Ok(s.to_owned()),
        }
    }
}
