use std::{
    fs::File,
    io::{BufReader, BufWriter},
    path::Path,
};

use oas3::OpenApiV3Spec;
use serde::{Deserialize, Serialize};

use crate::{
    args::import::Import,
    error::{Error, Result},
    specs::spec::Spec,
};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Specs(pub Vec<Spec>);

impl Specs {
    /// Loads specs from the given file based on the file extension.
    /// # Supported extensions:
    /// - `.yaml`, `.yml`
    /// - `.json`
    pub fn load(file: impl AsRef<Path>) -> Result<Self> {
        match file.as_ref().extension().and_then(|s| s.to_str()) {
            Some("yaml") | Some("yml") => Self::from_yaml(file),
            Some("json") => Self::from_json(file),
            _ => Err(Error::Msg("Unsupported file type".into())),
        }
    }

    /// Saves specs to the given file based on the file extension.
    /// # Supported extensions:
    /// - `.yaml`, `.yml`
    /// - `.json`
    pub fn save(&self, file: impl AsRef<Path>) -> Result<()> {
        match file.as_ref().extension().and_then(|s| s.to_str()) {
            Some("yaml") | Some("yml") => self.to_yaml(file),
            Some("json") => self.to_json(file),
            _ => Err(Error::Msg("Unsupported output file type".into())),
        }
    }

    /// Loads specs from the given yaml file
    pub fn from_yaml(file: impl AsRef<Path>) -> Result<Self> {
        let buffer = BufReader::new(File::open(file)?);
        serde_yaml::from_reader(buffer).map_err(Into::into)
    }

    /// Saves the specs into given yaml file
    pub fn to_yaml(&self, file: impl AsRef<Path>) -> Result<()> {
        let buffer = BufWriter::new(File::create(file)?);
        serde_yaml::to_writer(buffer, self).map_err(Into::into)
    }

    /// Loads specs from the given json file
    pub fn from_json(file: impl AsRef<Path>) -> Result<Self> {
        let buffer = BufReader::new(File::open(file)?);
        serde_json::from_reader(buffer).map_err(Into::into)
    }

    /// Saves the specs into given json file
    pub fn to_json(&self, file: impl AsRef<Path>) -> Result<()> {
        let buffer = BufWriter::new(File::create(file)?);
        serde_json::to_writer(buffer, self).map_err(Into::into)
    }
}

impl TryFrom<OpenApiV3Spec> for Specs {
    type Error = Error;

    fn try_from(
        value: OpenApiV3Spec,
    ) -> std::result::Result<Self, Self::Error> {
        Import::oas3_to_specs(value)
    }
}
