use pareg::{FromArg, Pareg};

use crate::error::{Error, Result};

pub mod action;
pub mod args_struct;
pub mod import;
pub mod serve;

fn missing_param_err(param: &str) -> Error {
    Error::Msg(format!("missing required argument: {param}"))
}

fn next_arg<'a, T: FromArg<'a>>(pareg: &'a mut Pareg) -> Result<T> {
    pareg.next();
    pareg.next_arg::<T>().map_err(|e| e.into())
}
