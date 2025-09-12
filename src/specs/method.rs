use serde::{Deserialize, Serialize};

use crate::error::Error;

/// Represents HTTP method used for HTTP requests
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Method {
    Get,
    Head,
    Post,
    Put,
    Delete,
    Connect,
    Options,
    Trace,
    Patch,
}

impl From<Method> for hyper::Method {
    fn from(value: Method) -> Self {
        match value {
            Method::Get => hyper::Method::GET,
            Method::Head => hyper::Method::HEAD,
            Method::Post => hyper::Method::POST,
            Method::Put => hyper::Method::PUT,
            Method::Delete => hyper::Method::DELETE,
            Method::Connect => hyper::Method::CONNECT,
            Method::Options => hyper::Method::OPTIONS,
            Method::Trace => hyper::Method::TRACE,
            Method::Patch => hyper::Method::PATCH,
        }
    }
}

impl TryFrom<hyper::Method> for Method {
    type Error = Error;

    fn try_from(value: hyper::Method) -> Result<Self, Self::Error> {
        Ok(match value {
            hyper::Method::GET => Method::Get,
            hyper::Method::HEAD => Method::Head,
            hyper::Method::POST => Method::Post,
            hyper::Method::PUT => Method::Put,
            hyper::Method::DELETE => Method::Delete,
            hyper::Method::CONNECT => Method::Connect,
            hyper::Method::OPTIONS => Method::Options,
            hyper::Method::TRACE => Method::Trace,
            hyper::Method::PATCH => Method::Patch,
            method => {
                return Err(Error::Msg(format!(
                    "unsupported HTTP method: {}",
                    method
                )));
            }
        })
    }
}
