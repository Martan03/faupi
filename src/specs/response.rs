use std::{collections::HashMap, iter::Peekable, str::Chars};

use http_body_util::Full;
use hyper::body::Bytes;
use serde::{Deserialize, Serialize};

use crate::{
    error::{self, Error},
    server::{
        HyperRes,
        url::{error::UrlError, var::UrlVar},
    },
    specs::status_code::StatusCode,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Response {
    pub status: StatusCode,
    pub body: serde_yaml::Value,
}

impl Response {
    pub fn expand_vars(
        &self,
        vars: &HashMap<String, UrlVar>,
    ) -> error::Result<serde_yaml::Value> {
        let mut body = self.body.clone();
        Self::sub_val(&mut body, vars)?;
        Ok(body)
    }

    fn sub_val(
        val: &mut serde_yaml::Value,
        vars: &HashMap<String, UrlVar>,
    ) -> error::Result<()> {
        match val {
            serde_yaml::Value::String(s) => Self::expand_val_vars(s, vars)?,
            serde_yaml::Value::Sequence(vals) => {
                for v in vals {
                    Self::sub_val(v, vars)?;
                }
            }
            serde_yaml::Value::Mapping(map) => {
                for (_, v) in map.iter_mut() {
                    Self::sub_val(v, vars)?;
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn expand_val_vars(
        s: &mut String,
        vars: &HashMap<String, UrlVar>,
    ) -> error::Result<()> {
        let mut res = String::new();
        let mut chars = s.chars().peekable();

        while let Some(c) = chars.next() {
            match c {
                '$' => {
                    let var = Self::read_var(&mut chars)?;
                    let val = vars
                        .get(&var)
                        .map_or(String::new(), ToString::to_string);
                    res.push_str(&val);
                }
                _ => res.push(c),
            }
        }

        *s = res;
        Ok(())
    }

    fn read_var(chars: &mut Peekable<Chars<'_>>) -> error::Result<String> {
        if chars.peek() != Some(&'{') {
            return Self::read_ident(chars);
        }

        _ = chars.next();
        let ident = Self::read_ident(chars)?;
        match chars.next() {
            Some('}') => Ok(ident),
            _ => Err(UrlError::UnclosedVar(ident).into()),
        }
    }

    fn read_ident(chars: &mut Peekable<Chars<'_>>) -> error::Result<String> {
        let mut ident = String::new();
        match chars.peek() {
            Some(c) if c.is_ascii_alphabetic() || *c == '_' => {
                ident.push(*c);
                chars.next();
            }
            Some(c) => return Err(UrlError::IdentStart(*c).into()),
            _ => return Err(UrlError::MissingIdent.into()),
        }

        while let Some(c) = chars.peek() {
            if !c.is_ascii_alphanumeric() && *c != '_' {
                break;
            }
            ident.push(*c);
            chars.next();
        }
        return Ok(ident);
    }
}

impl TryFrom<Response> for HyperRes {
    type Error = Error;

    fn try_from(value: Response) -> Result<Self, Self::Error> {
        Self::try_from(&value)
    }
}

impl TryFrom<&Response> for HyperRes {
    type Error = Error;

    fn try_from(value: &Response) -> Result<Self, Self::Error> {
        let body = serde_json::to_string(&value.body).unwrap_or("".into());
        hyper::Response::builder()
            .status(value.status.0)
            .header("content-type", "application/json")
            .body(Full::new(Bytes::from(body)))
            .map_err(Into::into)
    }
}
