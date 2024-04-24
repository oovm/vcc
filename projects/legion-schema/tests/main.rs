use legion_schema::{LegionPackageSetting, LegionWorkspaceSetting};
use schemars::{gen::SchemaSettings, JsonSchema};
use serde::Serialize;
use serde_json::{ser::PrettyFormatter, Serializer};
use std::path::Path;

#[test]
fn ready() {
    println!("it, works!")
}

#[test]
fn export_package() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    json_schema::<LegionPackageSetting>(here.join("tests").join("package.schema.json"));
    json_schema::<LegionWorkspaceSetting>(here.join("tests").join("workspace.schema.json"));
}

#[test]
fn export_local() {
    json_schema::<LegionPackageSetting>(r#"C:\Users\Dell\CLionProjects\valkyrie-monorepo\.config\.legion\legion.schema.json"#);
    json_schema::<LegionWorkspaceSetting>(
        r#"C:\Users\Dell\CLionProjects\valkyrie-monorepo\.config\.legion\legions.schema.json"#,
    );
}

fn json_schema<S>(path: impl AsRef<Path>)
where
    S: JsonSchema,
{
    let settings = SchemaSettings::draft07().with(|s| {
        s.option_nullable = true;
        s.option_add_null_type = false;
    });
    let gen = settings.into_generator();
    let schema = gen.into_root_schema_for::<S>();
    let mut file = std::fs::File::create(path).unwrap();
    let mut ser = Serializer::with_formatter(&mut file, PrettyFormatter::with_indent(b"    "));
    schema.serialize(&mut ser).unwrap();
    serde_json::to_writer_pretty(&mut file, &schema).unwrap()
}
