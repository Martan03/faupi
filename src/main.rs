use std::sync::Arc;
use tokio::sync::RwLock;

use crate::{
    error::Result,
    server::Server,
    specs::{specs_struct::Specs, watch_specs},
};

pub mod args;
pub mod error;
pub mod server;
pub mod specs;

#[tokio::main]
async fn main() -> Result<()> {
    let file = "test.yaml";
    let specs = Arc::new(RwLock::new(Specs::load(file)?));

    let _watcher = watch_specs(file, specs.clone())?;

    let server = Server::new(([127, 0, 0, 1], 3000), specs).await?;
    server.run().await
}
