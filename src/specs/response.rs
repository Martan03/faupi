use http_body_util::Full;
use hyper::body::Bytes;
use serde::{Deserialize, Serialize};

use crate::{error::Error, server::HyperRes, specs::status_code::StatusCode};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Response {
    pub status: StatusCode,
    pub body: serde_yaml::Value,
}

impl TryFrom<Response> for HyperRes {
    type Error = Error;

    fn try_from(value: Response) -> Result<Self, Self::Error> {
        let body = serde_json::to_string(&value.body).unwrap_or("".into());
        hyper::Response::builder()
            .status(value.status.0)
            .header("content-type", "application/json")
            .body(Full::new(Bytes::from(body)))
            .map_err(Into::into)
    }
}
