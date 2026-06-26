use std::{
    collections::HashMap,
    hash::{Hash, Hasher},
    iter::Peekable,
    str::Chars,
};

use crate::{
    error::{self, Error},
    server::url::{error::UrlError, var::UrlVar},
    specs::body::{
        Mapping, Sequence, TaggedValue,
        dynamic::{Dynamic, DynamicValue},
        type_constraint::TypeConstraint,
    },
};

/// Extends `serde_yaml::Value` by additional fields
#[derive(Debug, PartialEq, Clone, Default)]
pub enum Body {
    #[default]
    Null,
    Bool(bool),
    Number(serde_yaml::Number),
    String(String),
    Sequence(Sequence),
    Mapping(Mapping),
    Tagged(Box<TaggedValue>),
    Dynamic(Dynamic),
    Constraint(TypeConstraint),
}

impl Body {
    pub fn new(value: serde_yaml::Value) -> error::Result<Self> {
        Self::try_from(value)
    }

    pub fn resolve(
        &self,
        vars: &HashMap<String, UrlVar>,
        templates: &HashMap<String, Body>,
    ) -> serde_yaml::Value {
        match self {
            Body::Null => serde_yaml::Value::Null,
            Body::Bool(b) => serde_yaml::Value::Bool(*b),
            Body::Number(number) => serde_yaml::Value::Number(number.clone()),
            Body::String(s) => serde_yaml::Value::String(s.clone()),
            Body::Sequence(items) => {
                let mut new_items = vec![];
                for item in items {
                    new_items.push(item.resolve(vars, templates));
                }
                serde_yaml::Value::Sequence(new_items)
            }
            Body::Mapping(mapping) => {
                let mut new_map = serde_yaml::Mapping::new();
                for (k, v) in mapping.map.iter() {
                    new_map.insert(
                        k.resolve(vars, templates),
                        v.resolve(vars, templates),
                    );
                }
                serde_yaml::Value::Mapping(new_map)
            }
            Body::Tagged(tagged) => serde_yaml::Value::Tagged(Box::new(
                serde_yaml::value::TaggedValue {
                    tag: tagged.tag.clone(),
                    value: tagged.value.resolve(vars, templates),
                },
            )),
            Body::Dynamic(dynamic) => dynamic.resolve(vars, templates),
            Body::Constraint(constraint) => {
                if let Some(val) = &constraint.value {
                    val.resolve(vars, templates)
                } else {
                    serde_yaml::Value::Null
                }
            }
        }
    }

    pub fn validate(
        &self,
        inc: &serde_yaml::Value,
        vars: &HashMap<String, UrlVar>,
        templates: &HashMap<String, Body>,
    ) -> bool {
        match self {
            Body::Sequence(items) => {
                let serde_yaml::Value::Sequence(inc_seq) = inc else {
                    return false;
                };

                if items.len() != inc_seq.len() {
                    return false;
                }
                items
                    .iter()
                    .zip(inc_seq.iter())
                    .all(|(e, i)| e.validate(i, vars, templates))
            }
            Body::Mapping(mapping) => {
                let serde_yaml::Value::Mapping(inc_map) = inc else {
                    return false;
                };

                for (k, exp_v) in mapping.map.iter() {
                    let key = k.resolve(vars, templates);
                    if let Some(inc_v) = inc_map.get(&key) {
                        if !exp_v.validate(inc_v, vars, templates) {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }
                true
            }
            Body::Dynamic(dynamic) => dynamic.validate(inc, vars, templates),
            Body::Constraint(constraint) => {
                let same_type = match (constraint.exp_type.as_str(), inc) {
                    ("string", serde_yaml::Value::String(_)) => true,
                    ("number", serde_yaml::Value::Number(_)) => true,
                    ("boolean", serde_yaml::Value::Bool(_)) => true,
                    ("object", serde_yaml::Value::Mapping(_)) => true,
                    ("array", serde_yaml::Value::Sequence(_)) => true,
                    ("any", _) => true,
                    _ => false,
                };

                if !same_type {
                    return false;
                }

                if let Some(val) = &constraint.value {
                    return val.validate(inc, vars, templates);
                }
                true
            }
            _ => {
                let resolved = self.resolve(vars, templates);
                inc == &resolved
            }
        }
    }

    pub fn is_null(&self) -> bool {
        self == &Self::Null
    }
}

impl TryFrom<serde_yaml::Value> for Body {
    type Error = Error;

    fn try_from(value: serde_yaml::Value) -> Result<Self, Self::Error> {
        Ok(match value {
            serde_yaml::Value::Null => Self::Null,
            serde_yaml::Value::Bool(b) => Self::Bool(b),
            serde_yaml::Value::Number(numbers) => Self::Number(numbers),
            serde_yaml::Value::String(s) => Self::try_from(s)?,
            serde_yaml::Value::Sequence(values) => {
                let mut vals = vec![];
                for value in values {
                    vals.push(Self::try_from(value)?);
                }
                Self::Sequence(vals)
            }
            serde_yaml::Value::Mapping(map) => {
                let is_constraint = map.len() <= 2
                    && map.keys().all(|k| {
                        k.as_str() == Some("type")
                            || k.as_str() == Some("value")
                    });
                if is_constraint {
                    let type_val = map.get(&str_value("type")).unwrap();
                    let typ = type_val.as_str().unwrap_or("any");

                    let value = match map.get(&str_value("value")) {
                        Some(v) => Some(Box::new(Self::try_from(v.clone())?)),
                        None => None,
                    };
                    return Ok(Self::Constraint(TypeConstraint::new(
                        typ, value,
                    )));
                }

                let mut new_map = Mapping::new();
                for (k, v) in map {
                    new_map.insert(Self::try_from(k)?, Self::try_from(v)?);
                }
                Self::Mapping(new_map)
            }
            serde_yaml::Value::Tagged(tagged) => {
                Self::Tagged(Box::new(TaggedValue {
                    tag: tagged.tag,
                    value: Self::try_from(tagged.value)?,
                }))
            }
        })
    }
}

impl TryFrom<String> for Body {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let mut buffer = String::new();
        let mut res = Vec::new();

        let mut chars = value.chars().peekable();
        while let Some(c) = chars.next() {
            match c {
                '$' => {
                    if !buffer.is_empty() {
                        res.push(DynamicValue::Static(buffer));
                        buffer = String::new();
                    }
                    res.push(Self::read_var(&mut chars)?);
                }
                _ => buffer.push(c),
            }
        }

        if res.is_empty() {
            return Ok(Self::String(buffer));
        }
        Ok(Self::Dynamic(Dynamic::new(res)))
    }
}

impl Body {
    fn read_var(
        chars: &mut Peekable<Chars<'_>>,
    ) -> error::Result<DynamicValue> {
        if chars.peek() != Some(&'{') {
            return Self::read_var_inner(chars).map(|(_, v)| v);
        }

        _ = chars.next();
        let (ident, var) = Self::read_var_inner(chars)?;
        match chars.next() {
            Some('}') => Ok(var),
            _ => Err(UrlError::UnclosedVar(ident).into()),
        }
    }

    fn read_var_inner(
        chars: &mut Peekable<Chars<'_>>,
    ) -> error::Result<(String, DynamicValue)> {
        let ident = Self::read_ident(chars)?;
        if chars.peek() != Some(&'.') {
            return Ok((ident.clone(), DynamicValue::Var(ident)));
        }

        _ = chars.next();
        let attr = Self::read_ident(chars)?;
        match ident.as_str() {
            "fake" => Ok((ident, DynamicValue::Fake(attr))),
            "ref" => Ok((ident, DynamicValue::Ref(attr))),
            _ => Err(UrlError::UnknownObject(ident).into()),
        }
    }

    fn read_ident(chars: &mut Peekable<Chars<'_>>) -> error::Result<String> {
        let mut ident = String::new();
        match chars.peek() {
            Some(c) if c.is_ascii_alphabetic() || *c == '_' => {
                ident.push(*c);
                chars.next();
            }
            Some(c) => return Err(UrlError::IdentStart(*c).into()),
            _ => return Err(UrlError::MissingIdent.into()),
        }

        while let Some(c) = chars.peek() {
            if !c.is_ascii_alphanumeric() && *c != '_' {
                break;
            }
            ident.push(*c);
            chars.next();
        }
        return Ok(ident);
    }
}

impl From<Body> for serde_yaml::Value {
    fn from(value: Body) -> Self {
        Self::from(&value)
    }
}

impl From<&Body> for serde_yaml::Value {
    fn from(value: &Body) -> Self {
        match value {
            Body::Null => serde_yaml::Value::Null,
            Body::Bool(b) => serde_yaml::Value::Bool(*b),
            Body::Number(number) => serde_yaml::Value::Number(number.clone()),
            Body::String(s) => serde_yaml::Value::String(s.clone()),
            Body::Sequence(items) => {
                let mut new_items = vec![];
                for item in items {
                    new_items.push(Self::from(item));
                }
                serde_yaml::Value::Sequence(new_items)
            }
            Body::Mapping(mapping) => {
                let mut new_map = serde_yaml::Mapping::new();
                for (k, v) in mapping.map.iter() {
                    new_map.insert(Self::from(k), Self::from(v));
                }
                serde_yaml::Value::Mapping(new_map)
            }
            Body::Tagged(tagged) => serde_yaml::Value::Tagged(Box::new(
                serde_yaml::value::TaggedValue {
                    tag: tagged.tag.clone(),
                    value: Self::from(&tagged.value),
                },
            )),
            Body::Dynamic(dynamic) => {
                serde_yaml::Value::String(dynamic.to_string())
            }
            Body::Constraint(constraint) => {
                let mut map = serde_yaml::Mapping::new();
                map.insert(str_value("type"), str_value(&constraint.exp_type));
                if let Some(val) = &constraint.value {
                    map.insert(str_value("value"), Self::from(val.as_ref()));
                }

                serde_yaml::Value::Mapping(map)
            }
        }
    }
}

impl serde::Serialize for Body {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let value: serde_yaml::Value = self.clone().into();
        value.serialize(serializer)
    }
}

impl<'de> serde::Deserialize<'de> for Body {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = serde_yaml::Value::deserialize(deserializer)?;
        Body::try_from(value).map_err(serde::de::Error::custom)
    }
}

impl Hash for Body {
    fn hash<H: Hasher>(&self, state: &mut H) {
        std::mem::discriminant(self).hash(state);
        match self {
            Body::Null => {}
            Body::Bool(v) => v.hash(state),
            Body::Number(v) => v.hash(state),
            Body::String(v) => v.hash(state),
            Body::Sequence(v) => v.hash(state),
            Body::Mapping(v) => v.hash(state),
            Body::Tagged(v) => v.hash(state),
            Body::Dynamic(v) => v.hash(state),
            Body::Constraint(v) => v.hash(state),
        }
    }
}

impl Eq for Body {}

fn str_value(value: &str) -> serde_yaml::Value {
    serde_yaml::Value::String(value.to_string())
}
