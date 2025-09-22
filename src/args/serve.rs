use std::{path::PathBuf, sync::Arc};

use pareg::Pareg;
use tokio::sync::RwLock;

use crate::{
    args::{missing_param_err, next_arg},
    error::{Error, Result},
    server::{router::Router, server_struct::Server},
    specs::{mock_config::MockConfig, watch_specs},
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
        Serve::try_from(parsed)
    }

    pub async fn run(&self) -> Result<()> {
        let specs = MockConfig::load(&self.file)?;
        let router = Arc::new(RwLock::new(Router::new(specs)?));

        let _watcher = watch_specs(&self.file, router.clone())?;

        let server = Server::new((&self.server, self.port), router).await?;
        server.run().await
    }
}

impl TryFrom<ServeParser> for Serve {
    type Error = Error;

    fn try_from(value: ServeParser) -> Result<Self> {
        Ok(Serve {
            file: value.file.ok_or_else(|| missing_param_err("--spec"))?,
            server: value.server.unwrap_or("127.0.0.1".into()),
            port: value.port.unwrap_or(3000),
        })
    }
}
