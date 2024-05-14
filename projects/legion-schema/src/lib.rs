mod definitions;
mod dependencies;
mod errors;
pub(crate) mod examples;
mod inherits;

pub use crate::{
    definitions::{
        authors::{LegionPackageAuthors, LegionWorkspaceAuthors},
        category::CategoryType,
        license::{LegionPackageLicense, LegionWorkspaceLicense, LicenseType},
        versions::{LegionPackageVersion, LegionWorkspaceVersion},
    },
    dependencies::{DependencyInfo, DependencyInfoExtra},
    errors::{LegionConfigError, Result},
    inherits::InheritConfig,
    packages::LegionPackageSetting,
    workspace::LegionWorkspaceSetting,
};

mod global;
mod packages;
mod workspace;
