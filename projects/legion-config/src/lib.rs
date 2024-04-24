use serde::{Deserialize, Serialize};
pub use url::Url;

pub use crate::{versions::PackageVersion, workspace::Workspace};

mod application;
mod versions;
mod workspace;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegionConfig {
    pub workspace: Workspace,
}
