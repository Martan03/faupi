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
