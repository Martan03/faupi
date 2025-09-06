use std::sync::Arc;
use tokio::sync::RwLock;

use crate::{error::Result, server::Server, specs::Specs};

pub mod error;
pub mod server;
pub mod specs;

#[tokio::main]
async fn main() -> Result<()> {
    let specs = Arc::new(RwLock::new(Specs::load("test.yaml")));

    let server = Server::new(([127, 0, 0, 1], 3000), specs).await?;
    server.run().await
}
