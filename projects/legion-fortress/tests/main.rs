use legion_fortress::{LegionFortress, PackageInstall};
use semver::Version;

#[test]
fn ready() {
    println!("it, works!")
}

#[test]
fn export_package() -> legion_error::Result {
    let root = LegionFortress::new().unwrap();
    root.insert_path().unwrap();
    root.install_script(PackageInstall {
        names: vec!["valkyrie".to_string(), "legion".to_string()],
        version: Version { major: 0, minor: 0, patch: 4, pre: Default::default(), build: Default::default() },
    });
    root.install_script(PackageInstall {
        names: vec!["valkyrie".to_string(), "std".to_string()],
        version: Version { major: 0, minor: 0, patch: 4, pre: Default::default(), build: Default::default() },
    })
}
