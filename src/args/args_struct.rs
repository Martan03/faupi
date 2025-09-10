use pareg::Pareg;
use termal::printcln;

use crate::{
    args::{action::Action, serve::Serve},
    error::Result,
};

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Args {
    pub actions: Vec<Action>,
}

impl Args {
    pub const VERSION_NUMBER: &str = {
        let v = option_env!("CARGO_PKG_VERSION");
        if let Some(v) = v { v } else { "unknown" }
    };

    pub fn parse(mut args: Pareg) -> Result<Args> {
        let mut parsed = Self::default();

        while let Some(arg) = args.peek() {
            match arg {
                "s" | "serve" => {
                    args.next();
                    let serve = Serve::parse(&mut args)?;
                    parsed.actions.push(Action::Serve(serve));
                }
                "-h" | "--help" | "h" | "help" => {
                    args.next();
                    Self::help();
                }
                "-v" | "--version" => {
                    println!("faupi {}", Self::VERSION_NUMBER);
                    args.next();
                }
                arg => eprintln!("Unknown argument: '{arg}'"),
            }
        }

        Ok(parsed)
    }

    pub fn help() {
        printcln!(
            "Welcome to {'g}Faupi{'_} by {}{'_}
{'bl}Version {}{'_}

Blazingly fast API Mock Server written in Rust.

{'g}Usage{'_}:
  {'c}faupi{'_} [{'y}flags{'_}] [{'db}action{'_}]

{'g}Flags{'_}:
  {'y}-h  --help{'_}
    Displays this help.

  {'y}-v  --version{'_}
    Displays the version number of {'c}faupi{'_}.

{'g}Actions{'_}:
  {'db}s  serve{'_} {'bl}[serve arguments] [--]{'_}
    Creates the API mock server based on the arguments.

{'g}Serve arguments{'_}:
  {'y}-s  --spec{'_} <filepath>
    Path to the specification file.

  {'y}-a  --address{'_} <address>
    Mock API server address.

  {'y}-p  --port{'_} <port>
    Mock API server port.",
            termal::gradient("Martan03", (0, 220, 255), (175, 80, 255)),
            Self::VERSION_NUMBER
        );
    }
}
