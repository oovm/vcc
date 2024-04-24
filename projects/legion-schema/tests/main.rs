use legion_schema::{LegionPackageSetting, LegionWorkspaceSetting};
use schemars::gen::SchemaSettings;
use std::path::Path;

#[test]
fn ready() {
    println!("it, works!")
}

#[test]
fn export_package() {
    let settings = SchemaSettings::draft07().with(|s| {
        s.option_nullable = true;
        s.option_add_null_type = false;
    });
    let gen = settings.into_generator();
    let schema = gen.into_root_schema_for::<LegionPackageSetting>();
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let mut file = std::fs::File::create(here.join("tests").join("package.schema.json")).unwrap();
    serde_json::to_writer_pretty(&mut file, &schema).unwrap()
}

#[test]
fn export_workspace() {
    let settings = SchemaSettings::draft07().with(|s| {
        s.option_nullable = true;
        s.option_add_null_type = false;
    });
    let gen = settings.into_generator();
    let schema = gen.into_root_schema_for::<LegionWorkspaceSetting>();
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let mut file = std::fs::File::create(here.join("tests").join("workspace.schema.json")).unwrap();
    serde_json::to_writer_pretty(&mut file, &schema).unwrap()
}
