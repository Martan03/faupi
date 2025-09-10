use std::hash::{DefaultHasher, Hash, Hasher};

use indexmap::IndexMap;

use crate::specs::body::body::Body;

pub mod body;
pub mod dynamic;

pub type Sequence = Vec<Body>;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Mapping {
    map: IndexMap<Body, Body>,
}

impl Mapping {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, k: Body, v: Body) -> Option<Body> {
        self.map.insert(k, v)
    }
}

impl Hash for Mapping {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Hash the kv pairs in a way that is not sensitive to their order.
        let mut xor = 0;
        for (k, v) in self.map.iter() {
            let mut hasher = DefaultHasher::new();
            k.hash(&mut hasher);
            v.hash(&mut hasher);
            xor ^= hasher.finish();
        }
        xor.hash(state);
    }
}

#[derive(Clone, PartialEq, Hash, Debug)]
pub struct TaggedValue {
    pub tag: serde_yaml::value::Tag,
    pub value: Body,
}
