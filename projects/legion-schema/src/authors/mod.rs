use crate::InheritConfig;
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
