use std::{collections::HashMap, sync::Arc};

use http_body_util::Full;
use hyper::{Method, StatusCode, body::Bytes};
use tokio::sync::RwLock;

use crate::{
    error::Result,
    server::{HyperRes, router_node::RouterNode},
    specs::{spec::Spec, specs_struct::Specs},
};

pub type SharedRouter = Arc<RwLock<Router>>;

#[derive(Debug)]
pub struct Router {
    pub roots: HashMap<Method, RouterNode>,
    pub not_found: HyperRes,
}

impl Router {
    pub fn new(specs: Specs) -> Result<Self> {
        let mut router = Self::default();
        for spec in specs.0 {
            router.insert(spec)?;
        }
        Ok(router)
    }

    pub fn insert(&mut self, spec: Spec) -> Result<()> {
        let method = Method::from(spec.method);
        let root = self.roots.entry(method).or_default();

        let mut url_parts = spec.url.split("/");
        url_parts.next();
        root.insert(url_parts, HyperRes::try_from(spec.response)?);
        Ok(())
    }

    pub fn find(&self, method: &Method, url: &str) -> &HyperRes {
        self.find_opt(method, url).unwrap_or(&self.not_found)
    }

    pub fn find_opt(&self, method: &Method, url: &str) -> Option<&HyperRes> {
        let mut url_parts = url.split("/");
        url_parts.next();
        if let Some(root) = self.roots.get(method) {
            return root.find(url_parts);
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
