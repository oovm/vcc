use crate::InheritConfig;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use yearly_version::VersionRequest;

#[derive(Serialize, Deserialize, JsonSchema)]
#[schemars(untagged)]
pub enum DependencyInfo {
    Inherit(InheritConfig),
    Version(VersionRequest),
    Local(LocalDependency),
    Git(GitDependency),
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct LocalDependency {
    pub path: Option<String>,
    #[serde(flatten)]
    pub extra: DependencyInfoExtra,
}
#[derive(Serialize, Deserialize, JsonSchema)]
pub struct GitDependency {
    pub git: Option<String>,
    #[serde(flatten)]
    pub extra: DependencyInfoExtra,
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
