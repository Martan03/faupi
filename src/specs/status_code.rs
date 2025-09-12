use serde::{Deserialize, Serialize};

use crate::error::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusCode(pub u16);

impl TryFrom<StatusCode> for hyper::StatusCode {
    type Error = Error;

    fn try_from(value: StatusCode) -> Result<Self, Self::Error> {
        hyper::StatusCode::from_u16(value.0).map_err(|_| {
            Error::Msg(format!("Invalid status code: {}", value.0))
        })
    }
}

impl TryFrom<&str> for StatusCode {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse::<u16>().map(StatusCode).map_err(Into::into)
    }
}

impl Default for StatusCode {
    fn default() -> Self {
        Self(200)
    }
}
