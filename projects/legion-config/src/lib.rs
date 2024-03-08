use serde::{Deserialize, Serialize};

pub use crate::workspace::Workspace;

mod application;
mod workspace;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegionConfig {
    pub workspace: Workspace,
}
