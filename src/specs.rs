use std::{fs::File, io::BufReader, path::Path, sync::Arc};

use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

use crate::{
    error::{Error, Result},
    specs::spec::Spec,
};

pub mod method;
pub mod response;
pub mod spec;
pub mod status_code;

pub type SharedSpecs = Arc<RwLock<Specs>>;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
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

pub fn watch_specs(
    path: impl AsRef<Path>,
    specs: SharedSpecs,
) -> notify::Result<RecommendedWatcher> {
    let file = path.as_ref().to_owned();
    let mut watcher = notify::recommended_watcher(move |res| {
        let event: notify::Event = match res {
            Ok(event) => event,
            Err(e) => {
                eprintln!("watch error: {:?}", e);
                return;
            }
        };

        if event.kind.is_create() || event.kind.is_modify() {
            match Specs::load(&file) {
                Ok(new_specs) => {
                    let mut guard = specs.blocking_write();
                    *guard = new_specs;
                    println!("Config reloaded");
                }
                Err(e) => eprintln!("Failed to reload specs: {e}"),
            }
        }
    })?;

    watcher.watch(path.as_ref(), RecursiveMode::NonRecursive)?;
    Ok(watcher)
}

impl From<Specs> for SharedSpecs {
    fn from(value: Specs) -> Self {
        Arc::new(RwLock::new(value))
    }
}
