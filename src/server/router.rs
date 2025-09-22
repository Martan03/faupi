use std::{collections::HashMap, sync::Arc};

use http_body_util::Full;
use hyper::{Method, StatusCode, body::Bytes};
use tokio::sync::RwLock;

use crate::{
    error::Result,
    server::{
        HyperRes,
        router_node::RouterNode,
        url::{parser::UrlParser, var::UrlVar},
    },
    specs::{response::Response, spec::Spec, mock_config::MockConfig},
};

pub type SharedRouter = Arc<RwLock<Router>>;

#[derive(Debug)]
pub struct Router {
    pub roots: HashMap<Method, RouterNode>,
    pub not_found: HyperRes,
}

impl Router {
    /// Creates new Router tree based on the given specification
    pub fn new(specs: MockConfig) -> Result<Self> {
        let mut router = Self::default();
        for spec in specs.specs {
            router.insert(spec)?;
        }
        Ok(router)
    }

    /// Inserts route to the route tree and converts the spec response to hyper
    /// response.
    pub fn insert(&mut self, spec: Spec) -> Result<()> {
        let method = Method::from(spec.method);
        let root = self.roots.entry(method).or_default();

        let mut chars = spec.url.chars();
        let mut parser = UrlParser::new(&mut chars);
        _ = parser.next()?;
        root.insert(parser, spec.response)?;
        Ok(())
    }

    /// Finds a response in the response tree, returns None when finding fails
    pub fn find(
        &self,
        method: &Method,
        url: &str,
        vars: &mut HashMap<String, UrlVar>,
    ) -> Option<&Response> {
        let mut url_parts = url.split("/");
        url_parts.next();
        if let Some(root) = self.roots.get(method) {
            return root.find(url_parts, vars);
        }
        None
    }
}

impl Default for Router {
    fn default() -> Self {
        Self {
            roots: Default::default(),
            not_found: hyper::Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Full::new(Bytes::from("Not found")))
                .unwrap(),
        }
    }
}
