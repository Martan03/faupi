use serde::{Deserialize, Serialize};

use crate::specs::status_code::StatusCode;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Response {
    pub status: StatusCode,
    pub body: serde_yaml::Value,
}
