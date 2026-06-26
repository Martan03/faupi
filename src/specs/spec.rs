use serde::{Deserialize, Serialize};

use crate::{
    error::{Error, Result},
    specs::{body::body::Body, method::Method, response::EndpointResponse},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Spec {
    pub method: Method,
    pub url: String,
    #[serde(default)]
    pub request: Option<Body>,
    #[serde(default)]
    pub response: EndpointResponse,
}

impl Spec {
    pub fn validate(&self) -> Result<()> {
        match &self.method {
            Method::Post | Method::Put | Method::Patch => Ok(()),
            m if self.request.is_some() => Err(Error::Msg(format!(
                "The method '{:?}' cannot have a request body validator",
                m
            ))),
            _ => Ok(()),
        }
    }
}
