use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema)]
#[schemars(untagged)]
pub enum LicenseInfo {
    Single(LicenseType),
    Custom { file: String },
}

/// A predefined [spdx](https://spdx.org/licenses) license
#[derive(Default, Serialize, Deserialize, JsonSchema)]
pub enum LicenseType {
    /// <https://spdx.org/licenses/MIT.html>
    #[default]
    #[serde(rename = "MIT")]
    MIT,
    /// <https://spdx.org/licenses/GPL-3.0.html>
    #[serde(rename = "GPL-3.0")]
    GPL30,
    /// <https://spdx.org/licenses/Apache-2.0.html>
    #[serde(rename = "Apache-2.0")]
    Apache20,
}

impl Default for LicenseInfo {
    fn default() -> Self {
        Self::Single(LicenseType::default())
    }
}
