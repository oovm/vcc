use crate::InheritConfig;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, path::PathBuf};
use url::Url;
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
    // The local path of the dependency
    pub path: PathBuf,
    #[serde(flatten)]
    pub extra: DependencyInfoExtra,
}
#[derive(Serialize, Deserialize, JsonSchema)]
pub struct GitDependency {
    /// The git url of the dependency
    pub git: Url,
    /// The git tag of the dependency
    #[serde(default)]
    pub tag: String,
    /// The git branch of the dependency
    #[serde(default)]
    pub branch: String,
    /// The git revision of the dependency
    #[serde(default)]
    pub rev: Option<u64>,
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
