use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::specs::template::ValueType;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TemplateValue {
    Primitive {
        value: String,
        #[serde(rename = "type")]
        ty: ValueType,
    },
    Object(HashMap<String, TemplateValue>),
    Array(Vec<TemplateValue>),
    Null,
}
