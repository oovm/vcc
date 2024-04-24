use crate::InheritConfig;
use schemars::{
    gen::SchemaGenerator,
    schema::{Schema, SchemaObject, SubschemaValidation},
    JsonSchema,
};
use serde::{Deserialize, Serialize};

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
