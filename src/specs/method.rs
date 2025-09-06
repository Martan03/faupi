use serde::{Deserialize, Serialize};

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
