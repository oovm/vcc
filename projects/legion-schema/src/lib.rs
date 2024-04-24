use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

mod authors;
mod dependencies;
mod inherits;
mod licenses;
mod versions;
pub use crate::{
    authors::AuthorsInfo,
    dependencies::{DependencyInfo, DependencyInfoExtra},
    inherits::InheritConfig,
    licenses::{LicenseInfo, LicenseType},
    versions::VersionInfo,
};

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub struct LegionConfig {
    /// # [0-9a-zA-Z_-]+
    /// name of the package or workspace
    #[schemars(length(min = 1))]
    #[schemars(regex(pattern = r"[0-9a-zA-Z-]+"))]
    pub name: String,
    /// # workspace | a.b.c.d
    ///
    /// Inherit from workspace or use four-digit version number
    #[schemars(required)]
    pub version: VersionInfo,
    /// # bool
    ///
    /// Weather to publish this project, set  [`false`] to prevent accidental publishing
    #[serde(default)]
    pub publish: bool,
    /// # 0 <= string <= 144
    /// Briefly introduce this project in one sentence
    #[serde(default)]
    #[schemars(length(min = 0, max = 144))]
    pub description: String,
    /// # 0 <= keyword <= 9
    #[serde(default)]
    #[schemars(length(min = 0, max = 9))]
    // #[schemars(unique_items)]
    pub keywords: Vec<String>,
    /// # 0 <= category <= 5
    #[serde(default)]
    #[schemars(length(min = 0, max = 5))]
    // #[schemars(unique_items)]
    // #[schemars(items(enumDesc = "category"))]
    pub categories: Vec<String>,
    /// The name of authors
    #[serde(default)]
    pub authors: AuthorsInfo,
    #[serde(default)]
    #[schemars(url)]
    // #[schemars(default = "https://github.com/")]
    pub homepage: String,
    #[serde(default)]
    #[schemars(url)]
    // #[schemars(default = "https://github.com/")]
    pub repository: String,
    #[serde(default)]
    #[schemars(url)]
    // #[schemars(default = "https://docs.rs/")]
    pub document: String,
    /// # workspace | a.b.c.d
    /// The license used by the current project
    pub license: LicenseInfo,
    #[serde(default)]
    pub include: Vec<String>,
    #[serde(default)]
    // #[schemars(description = "works after include")]
    pub exclude: Vec<String>,
    #[serde(default)]
    pub dependency: BTreeMap<String, DependencyInfo>,
    #[serde(default)]
    pub dependency_debug: BTreeMap<String, DependencyInfo>,
    #[serde(default)]
    pub dependency_build: BTreeMap<String, DependencyInfo>,
    #[serde(default)]
    pub features: BTreeMap<String, Vec<String>>,
}
