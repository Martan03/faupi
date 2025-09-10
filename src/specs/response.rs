use std::collections::HashMap;

use serde::Deserialize;

use crate::{
    server::url::var::UrlVar,
    specs::{body::body::Body, status_code::StatusCode},
};

#[derive(Debug, Clone, Deserialize)]
pub struct Response {
    pub status: StatusCode,
    pub body: Body,
}

impl Response {
    pub fn expand_vars(
        &self,
        vars: &HashMap<String, UrlVar>,
    ) -> serde_yaml::Value {
        self.body.resolve(vars)
    }
}
