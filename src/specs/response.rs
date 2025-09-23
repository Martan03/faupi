use std::collections::HashMap;

use http_body_util::Full;
use hyper::body::Bytes;
use serde::{Deserialize, Serialize};

use crate::{
    error::Result,
    server::{HyperRes, url::var::UrlVar},
    specs::{body::body::Body, status_code::StatusCode},
};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Response {
    #[serde(default)]
    pub status: StatusCode,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delay: Option<u64>,
    #[serde(default, skip_serializing_if = "Body::is_null")]
    pub body: Body,
}

impl Response {
    pub fn to_http_response(
        &self,
        vars: &HashMap<String, UrlVar>,
        templates: &HashMap<String, Body>,
    ) -> Result<HyperRes> {
        let body = self.expand_vars(&vars, templates);
        let body = serde_json::to_string(&body).unwrap_or("".into());

        hyper::Response::builder()
            .status(self.status.0)
            .header("content-type", "application/json")
            .body(Full::new(Bytes::from(body)))
            .map_err(Into::into)
    }

    pub fn expand_vars(
        &self,
        vars: &HashMap<String, UrlVar>,
        templates: &HashMap<String, Body>,
    ) -> serde_yaml::Value {
        self.body.resolve(vars, templates)
    }
}
