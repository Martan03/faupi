use std::collections::HashMap;

use log::warn;
use serde_yaml::Value;

use crate::server::url::var::UrlVar;

#[derive(Debug, PartialEq, PartialOrd, Clone, Hash)]
pub struct Dynamic {
    pub values: Vec<DynamicValue>,
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Hash)]
pub enum DynamicValue {
    Static(String),
    Var(String),
}

impl Dynamic {
    pub fn new(values: Vec<DynamicValue>) -> Self {
        Self { values }
    }

    pub fn resolve(
        &self,
        vars: &HashMap<String, UrlVar>,
    ) -> serde_yaml::Value {
        if self.values.len() == 1
            && let DynamicValue::Var(ident) = &self.values[0]
            && let Some(var) = vars.get(ident)
        {
            return match var {
                UrlVar::String(s) => Value::String(s.clone()),
                UrlVar::Number(n) => Value::Number((*n).into()),
            };
        }

        let mut res = String::new();
        for value in self.values.iter() {
            match value {
                DynamicValue::Static(s) => res.push_str(s),
                DynamicValue::Var(var) => {
                    if let Some(val) = vars.get(var) {
                        res.push_str(&val.to_string());
                    } else {
                        warn!("Response variable `${var}` not defined.")
                    }
                }
            }
        }
        serde_yaml::Value::String(res)
    }
}
