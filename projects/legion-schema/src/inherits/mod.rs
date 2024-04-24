use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Inherit from other place
#[derive(Default, Serialize, Deserialize, JsonSchema)]
pub enum InheritConfig {
    /// Load config form workspace
    #[default]
    #[serde(rename = "workspace")]
    Workspace,
}
