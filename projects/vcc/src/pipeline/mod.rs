use crate::printer::print_wat;
use js_component_bindgen::{transpile, TranspileOpts};
use nyar_error::{NyarError, Result};
use std::{
    cell::RefCell,
    fs::{create_dir_all, File},
    io::Write,
    path::{Path, PathBuf},
    sync::Arc,
};
use wax::{Glob, LinkBehavior};

pub struct Pipeline {
    name: Arc<str>,
    debug_folder: PathBuf,
    release_folder: PathBuf,
    errors: RefCell<Vec<NyarError>>,
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
            errors: RefCell::new(vec![]),
        })
    }
    fn add_error<E: Into<NyarError>>(&self, error: E) {
        self.errors.borrow_mut().push(error.into())
    }
}

impl Pipeline {
    pub fn run(&self) -> Result<()> {
        self.run_optimizer()?;
        self.run_js_binding()?;
        self.run_dump_wat(&self.debug_folder)?;
        self.run_dump_wat(&self.release_folder)?;
        Ok(())
    }

    fn debug_file(&self) -> PathBuf {
        self.debug_folder.join(format!("{}.wasm", self.name))
    }
    fn release_file(&self) -> PathBuf {
        self.release_folder.join(format!("{}.wasm", self.name))
    }
    fn run_optimizer(&self) -> Result<()> {
        std::fs::copy(self.debug_file(), self.release_file())?;

        Ok(())
    }
}

impl Pipeline {
    fn run_js_binding(&self) -> Result<()> {
        let wit = std::fs::read(self.release_file())?;
        let config = TranspileOpts {
            name: self.name.to_string(),
            tla_compat: false,
            valid_lifting_optimization: true,
            no_nodejs_compat: true,
            ..Default::default()
        };
        match transpile(&wit, config) {
            Ok(out) => {
                for (name, bytes) in out.files {
                    std::fs::write(self.release_folder.join(name), bytes)?;
                }
                Ok(())
            }
            Err(e) => Err(NyarError::custom(e)),
        }
    }
}

impl Pipeline {
    fn run_dump_wat(&self, folder: &Path) -> Result<()> {
        let glob = Glob::new("*.{wasm}").expect("Internal Error: illegal wasm extension");
        let walker = glob
            .walk_with_behavior(folder, LinkBehavior::ReadTarget)
            .not(["**/secret/**"])
            .expect("Internal Error: illegal walk pattern");
        for entry in walker {
            let file = match entry {
                Ok(o) => o,
                Err(e) => {
                    self.add_error(NyarError::custom(e));
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
}
