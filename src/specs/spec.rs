use serde::{Deserialize, Serialize};

use crate::specs::{method::Method, response::Response};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Spec {
    pub method: Method,
    pub url: String,
    #[serde(default)]
    pub response: Response,
}
