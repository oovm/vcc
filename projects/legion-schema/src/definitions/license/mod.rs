use super::*;

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[schemars(untagged)]
pub enum LegionPackageLicense {
    /// inherit from workspace
    #[serde(rename = "workspace")]
    Workspace,
    Single(LicenseType),
    Custom(CustomLicense),
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[schemars(untagged)]
pub enum LegionWorkspaceLicense {
    Single(LicenseType),
    Custom(CustomLicense),
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct CustomLicense {
    /// Local path to the license file
    pub file: PathBuf,
}

/// A predefined [SPDX](https://spdx.org/licenses) license
#[derive(Default, Debug, Serialize, Deserialize, JsonSchema)]
pub enum LicenseType {
    /// [MIT License](https://spdx.org/licenses/MIT.html)
    #[default]
    #[serde(rename = "MIT")]
    MIT,
    /// [GNU General Public License v3.0](https://spdx.org/licenses/GPL-3.0.html)
    #[serde(rename = "GPL-3.0")]
    GPL30,
    /// [Apache License 2.0](https://spdx.org/licenses/Apache-2.0.html)
    #[serde(rename = "Apache-2.0")]
    Apache20,
    /// [BSD 3-Clause "New" or "Revised" License](https://spdx.org/licenses/BSD-3-Clause.html)
    #[serde(rename = "BSD-3-Clause")]
    BSD3Clause,
    /// [GNU Lesser General Public License v3.0](https://spdx.org/licenses/LGPL-3.0.html)
    #[serde(rename = "LGPL-3.0")]
    LGPL30,
    /// [European Union Public License 1.2](https://spdx.org/licenses/EUPL-1.2.html)
    #[serde(rename = "EUPL-1.2")]
    EUPL12,
    /// [Creative Commons Attribution 4.0 International](https://spdx.org/licenses/CC-BY-4.0.html)
    #[serde(rename = "CC-BY-4.0")]
    CCBy40,
    /// [Mozilla Public License 2.0](https://spdx.org/licenses/MPL-2.0.html)
    #[serde(rename = "MPL-2.0")]
    MPL20,
    /// [Eclipse Public License 2.0](https://spdx.org/licenses/EPL-2.0.html)
    #[serde(rename = "EPL-2.0")]
    EPL20,
    /// [BSD 2-Clause "Simplified" License](https://spdx.org/licenses/BSD-2-Clause.html)
    #[serde(rename = "BSD-2-Clause")]
    BSD2Clause,
    /// [GNU Affero General Public License v3.0](https://spdx.org/licenses/AGPL-3.0.html)
    #[serde(rename = "AGPL-3.0")]
    AGPL30,
    /// [Common Development and Distribution License 1.0](https://spdx.org/licenses/CDDL-1.0.html)
    #[serde(rename = "CDDL-1.0")]
    CDDL10,
    /// [Microsoft Public License](https://spdx.org/licenses/MS-PL.html)
    #[serde(rename = "MS-PL")]
    MSPL,
    /// [Artistic License 2.0](https://spdx.org/licenses/Artistic-2.0.html)
    #[serde(rename = "Artistic-2.0")]
    Artistic20,
    /// [zlib License](https://spdx.org/licenses/Zlib.html)
    #[serde(rename = "Zlib")]
    Zlib,
    /// [SIL Open Font License 1.1](https://spdx.org/licenses/OFL-1.1.html)
    #[serde(rename = "OFL-1.1")]
    OFL11,
    /// [ISC License](https://spdx.org/licenses/ISC.html)
    #[serde(rename = "ISC")]
    ISC,
    /// [Boost Software License 1.0](https://spdx.org/licenses/BSL-1.0.html)
    #[serde(rename = "BSL-1.0")]
    BSL10,
    /// [Microsoft Reciprocal License](https://spdx.org/licenses/MS-RL.html)
    #[serde(rename = "MS-RL")]
    MSRL,
    /// [Unlicense](https://spdx.org/licenses/Unlicense.html)
    #[serde(rename = "Unlicense")]
    Unlicense,
}

impl Default for LegionPackageLicense {
    fn default() -> Self {
        Self::Single(LicenseType::default())
    }
}
