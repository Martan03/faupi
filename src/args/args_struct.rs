use pareg::Pareg;

use crate::{
    args::{action::Action, serve_args::ServeArgs},
    error::Result,
};

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Args {
    pub actions: Vec<Action>,
}

impl Args {
    pub fn parse(mut args: Pareg) -> Result<Args> {
        let mut parsed = Self::default();

        while let Some(arg) = args.peek() {
            match arg {
                "serve" => {
                    args.next();
                    let serve_args = ServeArgs::parse(&mut args)?;
                    parsed.actions.push(Action::Serve(serve_args));
                }
                arg => eprintln!("Unknown argument: '{arg}'"),
            }
        }

        Ok(parsed)
    }
}
