use crate::InheritConfig;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, path::PathBuf};
use url::Url;
use yearly_version::VersionRequest;

#[derive(Serialize, Deserialize, JsonSchema)]
#[schemars(untagged)]
#[schemars(example = "dependency_examples")]
pub enum DependencyInfo {
    Inherit(InheritConfig),
    Version(VersionRequest),
    Local(LocalDependency),
    Git(GitDependency),
}

pub fn dependency_examples() -> Vec<DependencyInfo> {
    vec![
        DependencyInfo::Inherit(InheritConfig::Workspace),
        // DependencyInfo::Version(VersionRequest::default()),
        // DependencyInfo::Local(LocalDependency {
        //     path: PathBuf::from("./path/to/dependency"),
        //     extra: DependencyInfoExtra {
        //         version: VersionRequest::default(),
        //         optional: false,
        //         default_features: true,
        //         features: BTreeMap::new(),
        //     },
        // }),
        // DependencyInfo::Git(GitDependency {
        //     git: Url::from_str("git://github.com").unwrap(),
        //     tag: "".to_string(),
        //     branch: "".to_string(),
        //     rev: None,
        //     extra: DependencyInfoExtra {
        //         version: Default::default(),
        //         optional: false,
        //         default_features: false,
        //         features: Default::default(),
        //     },
        // }),
    ]
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
