use std::{collections::HashMap, fmt::Display};

use fake::locales::EN;
use log::warn;
use serde_yaml::Value;

use crate::{
    server::url::var::UrlVar,
    specs::body::{body::Body, fake::get_fake},
};

#[derive(Debug, PartialEq, PartialOrd, Clone, Hash)]
pub struct Dynamic {
    pub values: Vec<DynamicValue>,
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Hash)]
pub enum DynamicValue {
    Static(String),
    Var(String),
    Fake(String),
    Ref(String),
}

impl Dynamic {
    pub fn new(values: Vec<DynamicValue>) -> Self {
        Self { values }
    }

    pub fn resolve(
        &self,
        vars: &HashMap<String, UrlVar>,
        templates: &HashMap<String, Body>,
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
                        warn!("Response variable `${var}` not defined.");
                    }
                }
                DynamicValue::Fake(attr) => {
                    if let Some(val) = get_fake(attr, EN) {
                        res.push_str(&val.to_string());
                    } else {
                        warn!("Response variable `$fake.{attr}` not defined.");
                    }
                }
                DynamicValue::Ref(ref_name) => {
                    if let Some(body) = templates.get(ref_name) {
                        return body.resolve(vars, templates);
                    } else {
                        warn!("Template `$ref.{ref_name} not defined.");
                    }
                }
            }
        }
        serde_yaml::Value::String(res)
    }
}

impl Display for Dynamic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for value in self.values.iter() {
            match value {
                DynamicValue::Static(s) => write!(f, "{s}")?,
                DynamicValue::Var(ident) => write!(f, "${{{ident}}}")?,
                DynamicValue::Fake(attr) => write!(f, "${{fake.{attr}}}")?,
                DynamicValue::Ref(ref_name) => {
                    write!(f, "${{ref.{ref_name}}}")?
                }
            }
        }
        Ok(())
    }
}
