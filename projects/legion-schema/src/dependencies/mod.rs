use crate::InheritConfig;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use yearly_version::{Version, VersionRequest};

#[derive(Serialize, Deserialize, JsonSchema)]
#[schemars(untagged)]
pub enum DependencyInfo {
    Inherit(InheritConfig),
    Version(VersionRequest),
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
    pub version: VersionRequest,
    #[serde(default)]
    pub optional: bool,
    #[serde(default)]
    pub default_features: bool,
    #[serde(default)]
    pub features: BTreeMap<String, Vec<String>>,
}
