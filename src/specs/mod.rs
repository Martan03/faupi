use std::path::Path;

use notify::{RecommendedWatcher, RecursiveMode, Watcher};

use crate::{
    server::router::{Router, SharedRouter},
    specs::specs_struct::Specs,
};

pub mod body;
pub mod method;
pub mod response;
pub mod spec;
pub mod specs_struct;
pub mod status_code;

pub fn watch_specs(
    path: impl AsRef<Path>,
    router: SharedRouter,
) -> notify::Result<RecommendedWatcher> {
    let file = path.as_ref().to_owned();
    let mut watcher = notify::recommended_watcher(move |res| {
        let router = router.clone();
        let event: notify::Event = match res {
            Ok(event) => event,
            Err(e) => {
                eprintln!("watch error: {:?}", e);
                return;
            }
        };

        if event.kind.is_create() || event.kind.is_modify() {
            reload_specs(&file, router);
        }
    })?;

    watcher.watch(path.as_ref(), RecursiveMode::NonRecursive)?;
    Ok(watcher)
}

fn reload_specs(file: &Path, router: SharedRouter) {
    let specs = match Specs::load(file) {
        Ok(specs) => specs,
        Err(e) => {
            eprintln!("Failed to reload specs: {e}");
            return;
        }
    };

    match Router::new(specs) {
        Ok(new_router) => {
            let mut guard = router.blocking_write();
            *guard = new_router;
            println!("Specs reloaded");
        }
        Err(e) => eprintln!("Failed to re-generate router: {e}"),
    }
}
