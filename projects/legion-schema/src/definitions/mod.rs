use crate::{InheritConfig, LegionConfigError, LegionWorkspaceSetting, Result};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use yearly_version::Version;

pub mod authors;
pub mod category;
pub mod license;
pub mod versions;
