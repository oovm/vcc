use crate::InheritConfig;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Serialize, Deserialize, JsonSchema)]
#[schemars(untagged)]
pub enum DependencyInfo {
    Inherit(InheritConfig),
    Version(String),
    Local {
        path: Option<String>,
        #[serde(flatten)]
        extra: DependencyInfoExtra,
    },
    Git {
        git: Option<String>,
        #[serde(flatten)]
        extra: DependencyInfoExtra,
    },
}
#[derive(Serialize, Deserialize, JsonSchema)]
pub struct DependencyInfoExtra {
    // #[schemars(pattern = "^\\d{4}\\.\\d+\\.\\*$")]
    version: String,
    #[serde(default)]
    pub features: BTreeMap<String, Vec<String>>,
}
