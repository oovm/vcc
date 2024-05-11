use crate::{examples::*, DependencyInfo, LegionWorkspaceAuthors, LegionWorkspaceLicense, LegionWorkspaceVersion};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use url::Url;

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
    pub(crate) version: LegionWorkspaceVersion,
    /// # bool
    ///
    /// Weather to publish this project, set  [`false`] to prevent accidental publishing
    #[serde(default = "workspace_publish")]
    pub(crate) publish: bool,
    /// The name of authors
    #[serde(default)]
    pub(crate) authors: LegionWorkspaceAuthors,
    #[schemars(example = "homepage_example")]
    pub(crate) homepage: Option<Url>,
    #[schemars(example = "repository_example")]
    pub(crate) repository: Option<Url>,
    #[schemars(example = "document_example")]
    pub(crate) document: Option<Url>,
    /// # workspace | a.b.c.d
    /// The license used by the current project
    pub(crate) license: LegionWorkspaceLicense,
    /// # glob[]
    ///
    /// Publish with these files, works after exclude
    #[serde(default)]
    pub(crate) include: Vec<String>,
    /// # glob[]
    ///
    /// Don't publish these files
    #[serde(default)]
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
}

pub struct GlobPattern {}
