use serde::{Deserialize, Serialize};

pub mod template_value;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ValueType {
    String,
    Number,
    Bool,
    Object,
    Array,
}
