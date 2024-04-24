use crate::{examples::*, AuthorsInfo, DependencyInfo, LegionPackageLicense, LegionWorkspaceLicense, VersionInfo};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, BTreeSet},
    str::FromStr,
};
use url::Url;
use yearly_version::Version;

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub struct LegionWorkspaceSetting {
    /// # [0-9a-zA-Z_-]+
    /// name of the package or workspace
    #[schemars(length(min = 1))]
    #[schemars(regex(pattern = r"[0-9a-zA-Z-]+"))]
    pub(crate) name: String,
    /// # workspace | a.b.c.d
    ///
    /// Inherit from workspace or use four-digit version number
    #[schemars(required)]
    pub(crate) version: VersionInfo,
    /// # bool
    ///
    /// Weather to publish this project, set  [`false`] to prevent accidental publishing
    #[serde(default)]
    pub(crate) publish: bool,
    /// # 0 <= string <= 144
    /// Briefly introduce this project in one sentence
    #[serde(default)]
    #[schemars(length(min = 0, max = 144))]
    pub(crate) description: String,
    /// # 0 <= keyword <= 9
    #[serde(default)]
    #[schemars(length(min = 0, max = 9))]
    // #[schemars(unique_items)]
    pub(crate) keywords: Vec<String>,
    /// # 0 <= category <= 5
    #[serde(default)]
    #[schemars(length(min = 0, max = 5))]
    pub(crate) categories: BTreeSet<crate::CategoryType>,
    /// The name of authors
    #[serde(default)]
    pub(crate) authors: AuthorsInfo,
    #[schemars(example = "homepage_example")]
    pub(crate) homepage: Option<Url>,
    #[schemars(example = "repository_example")]
    pub(crate) repository: Option<Url>,
    #[schemars(example = "document_example")]
    pub(crate) document: Option<Url>,
    /// # workspace | a.b.c.d
    /// The license used by the current project
    pub(crate) license: LegionWorkspaceLicense,
    #[serde(default)]
    pub(crate) include: Vec<String>,
    #[serde(default)]
    // #[schemars(description = "works after include")]
    pub(crate) exclude: Vec<String>,
    #[serde(default)]
    pub(crate) dependency: BTreeMap<String, DependencyInfo>,
    #[serde(default)]
    pub(crate) dependency_debug: BTreeMap<String, DependencyInfo>,
    #[serde(default)]
    pub(crate) dependency_build: BTreeMap<String, DependencyInfo>,
    #[serde(default)]
    pub(crate) scripts: BTreeMap<String, String>,
    #[serde(default)]
    pub(crate) default_script: Option<String>,
    #[serde(default)]
    pub(crate) features: BTreeMap<String, Vec<String>>,
}
