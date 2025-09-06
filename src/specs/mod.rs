use std::path::Path;

use notify::{RecommendedWatcher, RecursiveMode, Watcher};

use crate::specs::specs_struct::{SharedSpecs, Specs};

pub mod method;
pub mod response;
pub mod spec;
pub mod specs_struct;
pub mod status_code;

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
