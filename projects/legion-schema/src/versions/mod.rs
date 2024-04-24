use crate::InheritConfig;
use schemars::{
    JsonSchema,
    _private::{apply_metadata, MaybeSerializeWrapper},
    gen::SchemaGenerator,
    schema::{Metadata, Schema, SchemaObject, SubschemaValidation},
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{borrow::Cow, str::FromStr};
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
