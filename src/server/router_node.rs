use std::{collections::HashMap, str::Split};

use crate::{
    error::Result,
    server::{
        endpoint::Endpoint,
        url::{parser::UrlParser, segment::UrlSegment, var::UrlVar},
    },
};

/// Node in the router tree
#[derive(Debug, Default)]
pub struct RouterNode {
    pub children: HashMap<String, RouterNode>,
    pub dyn_children: Vec<(UrlSegment, RouterNode)>,
    pub endpoint: Option<Endpoint>,
}

impl RouterNode {
    /// Inserts the given response to the router tree. When this node is a final
    /// node, sets its response, otherwise continues traversing.
    pub fn insert(&mut self, mut url: UrlParser, ep: Endpoint) -> Result<()> {
        let Some(segment) = url.next()? else {
            self.endpoint = Some(ep);
            return Ok(());
        };

        if let Some(s) = segment.get_static() {
            let node = self.children.entry(s.to_owned()).or_default();
            node.insert(url, ep)?;
        } else {
            let mut node = RouterNode::default();
            node.insert(url, ep)?;
            self.dyn_children.push((segment, node));
        }
        Ok(())
    }

    /// Finds a response for a given URL. Returns nodes response when current
    /// node is final, otherwise continues traversing.
    pub fn find(
        &self,
        mut url: Split<'_, &str>,
        vars: &mut HashMap<String, UrlVar>,
    ) -> Option<&Endpoint> {
        let Some(part) = url.next() else {
            return self.endpoint.as_ref();
        };

        if let Some(node) = self.children.get(part) {
            return node.find(url, vars);
        }

        for (segment, node) in self.dyn_children.iter() {
            if segment.matches(part, vars) {
                return node.find(url, vars);
            }
        }
        None
    }
}
