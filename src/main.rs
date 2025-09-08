use pareg::Pareg;
use std::process::ExitCode;
use termal::eprintcln;
use tokio::task::JoinSet;

use crate::{
    args::{action::Action, args_struct::Args},
    error::Result,
};

pub mod args;
pub mod error;
pub mod server;
pub mod specs;

#[tokio::main]
async fn main() -> ExitCode {
    match run().await {
        Ok(_) => ExitCode::SUCCESS,
        Err(e) => {
            eprintcln!("{'r}Error{'_}: {}", e);
            ExitCode::FAILURE
        }
    }
}

async fn run() -> Result<()> {
    let args = Args::parse(Pareg::args())?;
    let mut set = JoinSet::new();
    for action in args.actions {
        match action {
            Action::Serve(s) => _ = set.spawn(async move { s.run().await }),
        }
    }

    while let Some(res) = set.join_next().await {
        res??;
    }

    Ok(())
}
