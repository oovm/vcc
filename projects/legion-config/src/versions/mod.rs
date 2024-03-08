use std::str::FromStr;

#[derive(Clone, Debug)]
pub struct PackageVersion {
    name: String,
    version: Option<VersionReq>,
}

impl FromStr for PackageVersion {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_once("=") {
            Some((name, version)) => match VersionReq::from_str(version) {
                Ok(o) => Ok(PackageVersion { name: name.to_string(), version: Some(o) }),
                Err(e) => Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, e)),
            },
            None => Ok(PackageVersion { name: s.to_string(), version: None }),
        }
    }
}
