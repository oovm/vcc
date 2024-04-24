use super::*;

/// The predefined category type
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, JsonSchema)]
pub enum CategoryType {
    #[serde(rename = "compiler")]
    Compiler,
    #[serde(rename = "game")]
    Game,
    #[serde(rename = "graphics")]
    Graphics,
    #[serde(rename = "network")]
    Network,
    #[serde(rename = "audio")]
    Audio,
    #[serde(rename = "database")]
    Database,
    #[serde(rename = "filesystem")]
    Filesystem,
}
