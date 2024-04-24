use super::*;

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[schemars(untagged)]
pub enum LegionPackageVersion {
    Inherit(InheritConfig),
    Yearly(Version),
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[schemars(untagged)]
pub enum LegionWorkspaceVersion {
    Yearly(Version),
}

impl LegionPackageVersion {
    pub fn seek(&self, ws: &Option<LegionWorkspaceSetting>) -> Result<Version> {
        match self {
            Self::Inherit(InheritConfig::Workspace) => match ws {
                Some(s) => s.version.seek(),
                None => Err(LegionConfigError::MissingField { field: "workspace.version" }),
            },
            Self::Yearly(y) => Ok(*y),
        }
    }
}

impl LegionWorkspaceVersion {
    pub fn seek(&self) -> Result<Version> {
        match self {
            Self::Yearly(v) => Ok(*v),
        }
    }
}
