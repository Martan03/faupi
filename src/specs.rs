use std::{fs::File, io::BufReader, path::Path};

use serde::{Deserialize, Serialize};

use crate::{
    error::{Error, Result},
    specs::spec::Spec,
};

pub mod method;
pub mod response;
pub mod spec;
pub mod status_code;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Specs(Vec<Spec>);

impl Specs {
    /// Loads specs from the given file based on the file extension.
    /// # Supported extensions:
    /// - `.yaml`, `.yml`
    /// - `.json`
    pub fn load(file: impl AsRef<Path>) -> Self {
        let res = match file.as_ref().extension().and_then(|s| s.to_str()) {
            Some("yaml") | Some("yml") => Specs::from_yaml(file),
            Some("json") => Specs::from_json(file),
            _ => Err(Error::Msg("Unsupported file type".into())),
        };
        res.unwrap_or_default()
    }

    /// Loads specs from the given yaml file
    pub fn from_yaml(file: impl AsRef<Path>) -> Result<Self> {
        let buffer = BufReader::new(File::open(file)?);
        Ok(serde_yaml::from_reader(buffer)?)
    }

    /// Loads specs from the given json file
    pub fn from_json(file: impl AsRef<Path>) -> Result<Self> {
        let buffer = BufReader::new(File::open(file)?);
        Ok(serde_json::from_reader(buffer)?)
    }
}
