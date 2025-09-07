use std::{path::PathBuf, sync::Arc};

use pareg::Pareg;
use tokio::sync::RwLock;

use crate::{
    args::{missing_param_err, next_arg},
    error::Result,
    server::Server,
    specs::{specs_struct::Specs, watch_specs},
};

#[derive(Debug, Clone, PartialEq)]
pub struct Serve {
    // Specification file path
    pub file: PathBuf,
    // API mock server address
    pub server: String,
    // API mock server port
    pub port: u16,
}

#[derive(Debug, Default)]
struct ServeParser {
    file: Option<PathBuf>,
    server: Option<String>,
    port: Option<u16>,
}

impl Serve {
    pub fn parse(args: &mut Pareg) -> Result<Serve> {
        let mut parsed = ServeParser::default();
        while let Some(arg) = args.peek() {
            match arg {
                "-s" | "--spec" => parsed.file = Some(next_arg(args)?),
                "-a" | "--address" => parsed.server = Some(next_arg(args)?),
                "-p" | "--port" => parsed.port = Some(next_arg(args)?),
                "--" => {
                    args.next();
                    break;
                }
                _ => break,
            }
        }
        parsed.build()
    }

    pub async fn run(&self) -> Result<()> {
        let specs = Arc::new(RwLock::new(Specs::load(&self.file)?));

        let _watcher = watch_specs(&self.file, specs.clone())?;

        let server = Server::new((&self.server, self.port), specs).await?;
        server.run().await
    }
}

impl ServeParser {
    fn build(self) -> Result<Serve> {
        Ok(Serve {
            file: self.file.ok_or_else(|| missing_param_err("--spec"))?,
            server: self.server.unwrap_or("127.0.0.1".into()),
            port: self.port.unwrap_or(3000),
        })
    }
}
