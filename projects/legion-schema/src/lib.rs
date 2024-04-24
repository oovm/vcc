mod authors;
mod definitions;
mod dependencies;
mod errors;
pub(crate) mod examples;
mod inherits;
mod versions;
pub use crate::{
    authors::AuthorsInfo,
    definitions::{
        category::CategoryType,
        license::{LegionPackageLicense, LegionWorkspaceLicense, LicenseType},
    },
    dependencies::{DependencyInfo, DependencyInfoExtra},
    errors::{LegionConfigError, Result},
    inherits::InheritConfig,
    packages::LegionPackageSetting,
    versions::VersionInfo,
    workspace::LegionWorkspaceSetting,
};

mod packages;
mod workspace;
