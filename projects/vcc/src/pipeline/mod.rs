use crate::printer::print_wat;
use nyar_error::{NyarError, Result};
use std::{
    fs::{copy, create_dir_all, File},
    io::Write,
    ops::AddAssign,
    path::{Path, PathBuf},
    sync::Arc,
};
use wax::{Glob, LinkBehavior};
pub struct Pipeline {
    name: Arc<str>,
    debug_folder: PathBuf,
    release_folder: PathBuf,
    errors: Vec<NyarError>,
}

impl Pipeline {
    pub fn new(name: Arc<str>, workspace: &Path) -> Result<Self> {
        let debug_folder = workspace.join("target/valkyrie/debug");
        create_dir_all(&debug_folder)?;
        let release_folder = workspace.join("target/valkyrie/release");
        let release_interfaces = release_folder.join("interfaces");
        create_dir_all(&release_interfaces)?;
        Ok(Self {
            name,
            debug_folder: debug_folder.canonicalize()?,
            release_folder: release_folder.canonicalize()?,
            errors: vec![],
        })
    }
}

impl AddAssign<NyarError> for Pipeline {
    fn add_assign(&mut self, rhs: NyarError) {
        self.errors.push(rhs)
    }
}

impl Pipeline {
    pub fn run(&self) -> Result<()> {
        self.run_optimizer()?;
        Ok(())
    }

    fn wasm_to_wast(&mut self, folder: &Path) -> Result<()> {
        let glob = Glob::new("*.{wasm}").expect("Internal Error: illegal wasm extension");
        let walker = glob
            .walk_with_behavior(folder, LinkBehavior::ReadTarget)
            .not(["**/secret/**"])
            .expect("Internal Error: illegal walk pattern");
        for entry in walker {
            let file = match entry {
                Ok(o) => o,
                Err(e) => {
                    self.errors.push(NyarError::custom(e));
                    continue;
                }
            };
            let path = file.path();
            match print_wat(&path)? {
                Some(s) => {
                    let mut file = File::create(path.with_extension("wat"))?;
                    file.write_all(s.as_bytes())?;
                }
                None => continue,
            }
        }
        Ok(())
    }
    fn debug_file(&self) -> PathBuf {
        self.debug_folder.join(format!("{}.wasm", self.name))
    }
    fn release_file(&self) -> PathBuf {
        self.release_folder.join(format!("{}.wasm", self.name))
    }
    fn run_optimizer(&self) -> Result<()> {
        copy(self.debug_file(), self.release_file())?;

        Ok(())
    }
}

impl Pipeline {
    fn run_js_binding(&self) -> Result<()> {
        Ok(())
    }
}

#[test]
fn test() {
    let mut builder = Pipeline::new(Arc::from("valkyrie_wit"), &Path::new(env!("CARGO_MANIFEST_DIR"))).unwrap();
    builder.run().unwrap();

    // let wit = read(out_dir.join("valkyrie_wit.wasm")).unwrap();
    // let out = transpile(&wit, TranspileOpts { name: "valkyrie_wit".to_string(), ..Default::default() }).unwrap();
    //
    // for (name, bytes) in out.files {
    //     std::fs::write(out_dir.join(name), bytes).unwrap();
    // }
}
