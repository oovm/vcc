use legion_fortress::{InstallRequest, LegionFortress};
use semver::Version;

#[test]
fn ready() {
    println!("it, works!")
}

#[test]
fn export_package() {
    let root = LegionFortress::new().unwrap();
    root.insert_path().unwrap();
    root.install_wasm(InstallRequest {
        names: vec!["valkyrie".to_string(), "legion".to_string()],
        version: Version { major: 0, minor: 0, patch: 4, pre: Default::default(), build: Default::default() },
        alias: "".to_string(),
    })
    .unwrap();
    root.install_wasm(InstallRequest {
        names: vec!["valkyrie".to_string(), "std".to_string()],
        version: Version { major: 0, minor: 0, patch: 4, pre: Default::default(), build: Default::default() },
        alias: "".to_string(),
    })
    .unwrap();
}
