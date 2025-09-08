use http_body_util::Full;
use hyper::body::Bytes;

pub mod router;
pub mod router_node;
pub mod server_struct;

pub type HyperRes = hyper::Response<Full<Bytes>>;
