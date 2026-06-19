mod multi_response;
mod response;
mod strategy;

pub use multi_response::MultiResponse;
pub use response::Response;
pub use strategy::Strategy;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EndpointResponse {
    Multi(MultiResponse),
    Single(Response),
}

impl EndpointResponse {
    /// Gets the endpoint response.
    ///
    /// When multiple response, picks based on the set strategy.
    pub fn get(&self) -> &Response {
        match self {
            EndpointResponse::Single(res) => res,
            EndpointResponse::Multi(multi) => multi.get(),
        }
    }
}

impl Default for EndpointResponse {
    fn default() -> Self {
        Self::Single(Response::default())
    }
}
