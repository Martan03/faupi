use std::{collections::HashMap, str::Split};

use crate::server::HyperRes;

#[derive(Debug, Default)]
pub struct RouterNode {
    pub children: HashMap<String, RouterNode>,
    pub response: Option<HyperRes>,
}

impl RouterNode {
    pub fn insert(&mut self, mut url: Split<'_, &str>, res: HyperRes) {
        let Some(part) = url.next() else {
            self.response = Some(res);
            return;
        };

        let node = self.children.entry(part.to_owned()).or_default();
        node.insert(url, res);
    }

    pub fn find(&self, mut url: Split<'_, &str>) -> Option<&HyperRes> {
        let Some(part) = url.next() else {
            return self.response.as_ref();
        };

        if let Some(node) = self.children.get(part) {
            return node.find(url);
        }
        None
    }
}
