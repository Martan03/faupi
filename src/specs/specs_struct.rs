use std::{fs::File, io::BufReader, path::Path};

use serde::{Deserialize};

use crate::{
    error::{Error, Result},
    specs::spec::Spec,
};

#[derive(Debug, Default, Clone, Deserialize)]
pub struct Specs(pub Vec<Spec>);

impl Specs {
    /// Loads specs from the given file based on the file extension.
    /// # Supported extensions:
    /// - `.yaml`, `.yml`
    /// - `.json`
    pub fn load(file: impl AsRef<Path>) -> Result<Self> {
        match file.as_ref().extension().and_then(|s| s.to_str()) {
            Some("yaml") | Some("yml") => Specs::from_yaml(file),
            Some("json") => Specs::from_json(file),
            _ => Err(Error::Msg("Unsupported file type".into())),
        }
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
