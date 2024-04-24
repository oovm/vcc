use schemars::{schema_for, JsonSchema};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

mod authors;

pub use crate::authors::AuthorsInfo;

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct LegionConfig {
    /// name of the package
    // #[schemars(pattern = "[0-9a-zA-Z_-]+")]
    #[schemars(length(min = 1))]
    pub name: String,

    /// # workspace | a.b.c.d
    ///
    /// Inherit from workspace or use four-digit version number
    // #[schemars(pattern = "^workspace$|^\\d+\\.\\d+\\.\\d+\\.\\d+$")]
    // #[schemars(default = "0.0.0.0")]
    pub version: String,

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

    #[serde(default)]
    /// The name of authors
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

    // #[schemars(enumDesc = "one of ")]
    // #[schemars(default = "MIT")]
    pub license: LicenseInfo,

    #[serde(default)]
    pub include: Vec<String>,

    #[serde(default)]
    // #[schemars(description = "works after include")]
    pub exclude: Vec<String>,

    #[serde(default)]
    pub dependency: BTreeMap<String, DependencyInfo>,

    #[serde(default)]
    pub dependency_development: BTreeMap<String, DependencyInfo>,

    #[serde(default)]
    pub dependency_build: BTreeMap<String, DependencyInfo>,

    #[serde(default)]
    pub features: BTreeMap<String, Vec<String>>,
}

#[derive(Serialize, Deserialize, JsonSchema)]
#[schemars(untagged)]
pub enum DependencyInfo {
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

#[derive(Serialize, Deserialize, JsonSchema)]
#[schemars(untagged)]
pub enum LicenseInfo {
    Single(LicenseType),
    Custom { file: String },
}

// #[derive(schemars::JsonSchema)]
pub enum LicenseType {
    /// The kind is one!
    One,
    /// The kind is two!
    Two,
    /// The kind is not three!
    NotThree,
}

#[test]
fn test() {
    let schema = schema_for!(LegionConfig);
    println!("{}", serde_json::to_string_pretty(&schema).unwrap());
}
