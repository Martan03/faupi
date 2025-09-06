use std::path::PathBuf;

use pareg::Pareg;

use crate::{
    args::{missing_param_err, next_arg},
    error::Result,
};

#[derive(Debug, Clone, PartialEq)]
pub struct ServeArgs {
    // Specification file path
    pub file: PathBuf,
    // API mock server address
    pub server: String,
    // API mock server port
    pub port: u16,
}

#[derive(Debug, Default)]
struct ServeArgsParser {
    file: Option<PathBuf>,
    server: Option<String>,
    port: Option<u16>,
}

impl ServeArgs {
    pub fn parse(args: &mut Pareg) -> Result<ServeArgs> {
        let mut parsed = ServeArgsParser::default();
        while let Some(arg) = args.peek() {
            match arg {
                "-s" | "--spec" => parsed.file = Some(next_arg(args)?),
                "-a" | "--address" => parsed.server = Some(next_arg(args)?),
                "-p" | "--port" => parsed.port = Some(next_arg(args)?),
                _ => break,
            }
        }
        parsed.build()
    }
}

impl ServeArgsParser {
    fn build(self) -> Result<ServeArgs> {
        Ok(ServeArgs {
            file: self.file.ok_or_else(|| missing_param_err("--spec"))?,
            server: self.server.unwrap_or("127.0.0.1".into()),
            port: self.port.unwrap_or(3000),
        })
    }
}
