use super::*;

#[derive(Serialize, Deserialize, JsonSchema)]
#[schemars(untagged)]
pub enum LegionPackageAuthors {
    Inherit(InheritConfig),
    Single(String),
    Multiple(Vec<String>),
}

#[derive(Serialize, Deserialize, JsonSchema)]
#[schemars(untagged)]
pub enum LegionWorkspaceAuthors {
    Single(String),
    Multiple(Vec<String>),
}

impl Default for LegionPackageAuthors {
    fn default() -> Self {
        Self::Multiple(vec![])
    }
}

impl Default for LegionWorkspaceAuthors {
    fn default() -> Self {
        Self::Multiple(vec![])
    }
}

impl LegionPackageAuthors {
    /// Seek the authors from package and workspace
    pub fn seek(&self, ws: &Option<LegionWorkspaceSetting>) -> Result<Vec<String>> {
        match self {
            Self::Inherit(InheritConfig::Workspace) => match ws {
                Some(ws) => ws.authors.seek(),
                None => Ok(vec![]),
            },
            Self::Single(s) => Ok(vec![s.to_owned()]),
            Self::Multiple(s) => Ok(s.to_owned()),
        }
    }
}
impl LegionWorkspaceAuthors {
    /// Seek the authors from workspace
    pub fn seek(&self) -> Result<Vec<String>> {
        match self {
            Self::Single(s) => Ok(vec![s.to_owned()]),
            Self::Multiple(s) => Ok(s.to_owned()),
        }
    }
}
