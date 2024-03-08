use std::fmt::Formatter;

use serde::{
    de::{MapAccess, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
};
use serde_types::OneOrMany;

mod der;
mod ser;

#[derive(Debug, Clone)]
pub struct Workspace {
    /// Whether the workspace mode is enabled
    pub enable: bool,
    pub include: Vec<String>,
    pub exclude: Vec<String>,
}

impl Workspace {
    pub fn enable(&mut self) {}
}

impl Default for Workspace {
    fn default() -> Self {
        Self { enable: false, include: vec![], exclude: vec![] }
    }
}
