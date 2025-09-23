use std::path::Path;

use log::{error, info};
use notify::{RecommendedWatcher, RecursiveMode, Watcher};

use crate::{
    server::router::{Router, SharedRouter},
    specs::mock_config::MockConfig,
};

pub mod body;
pub mod method;
pub mod mock_config;
pub mod response;
pub mod spec;
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
                error!("Watching specification: {e}.");
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
    let specs = match MockConfig::load(file) {
        Ok(specs) => specs,
        Err(e) => {
            error!("Reloading specification: {e}.");
            return;
        }
    };

    match Router::new(specs) {
        Ok(new_router) => {
            let mut guard = router.blocking_write();
            *guard = new_router;
            info!("Specification reloaded.");
        }
        Err(e) => error!("Regenerating router: {e}"),
    }
}
