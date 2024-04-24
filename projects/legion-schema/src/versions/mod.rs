use crate::{InheritConfig, LegionConfigError, LegionPackageSetting};
use schemars::{
    gen::SchemaGenerator,
    schema::{Schema, SchemaObject, SubschemaValidation},
    JsonSchema,
};
use serde::{Deserialize, Serialize};

use crate::Result;
use std::borrow::Cow;
use yearly_version::Version;

#[derive(Serialize, Deserialize)]
pub enum VersionInfo {
    Inherit(InheritConfig),
    Yearly(Version),
}

impl JsonSchema for VersionInfo {
    fn schema_name() -> String {
        "VersionInfo".to_owned()
    }
    fn schema_id() -> Cow<'static, str> {
        Cow::Borrowed(concat!(module_path!(), "::", "VersionInfo"))
    }
    fn json_schema(gen: &mut SchemaGenerator) -> Schema {
        Schema::Object(SchemaObject {
            subschemas: Some(Box::new(SubschemaValidation {
                one_of: Some(vec![gen.subschema_for::<InheritConfig>(), gen.subschema_for::<Version>()]),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
impl VersionInfo {
    pub fn seek(&self, ws: &Option<LegionPackageSetting>) -> Result<Version> {
        match self {
            Self::Inherit(InheritConfig::Workspace) => match ws {
                Some(s) => match &s.version {
                    Self::Inherit(_) => Err(LegionConfigError::NotInheritable { field: "workspace.version" }),
                    Self::Yearly(y) => Ok(*y),
                },
                None => Err(LegionConfigError::MissingField { field: "workspace.version" }),
            },
            Self::Yearly(y) => Ok(*y),
        }
    }
}
