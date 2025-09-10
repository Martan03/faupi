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
}

impl Body {
    pub fn new(value: serde_yaml::Value) -> error::Result<Self> {
        Self::try_from(value)
    }

    pub fn resolve(
        &self,
        vars: &HashMap<String, UrlVar>,
    ) -> serde_yaml::Value {
        match self {
            Body::Null => serde_yaml::Value::Null,
            Body::Bool(b) => serde_yaml::Value::Bool(*b),
            Body::Number(number) => serde_yaml::Value::Number(number.clone()),
            Body::String(s) => serde_yaml::Value::String(s.clone()),
            Body::Sequence(items) => {
                let mut new_items = vec![];
                for item in items {
                    new_items.push(item.resolve(vars));
                }
                serde_yaml::Value::Sequence(new_items)
            }
            Body::Mapping(mapping) => {
                let mut new_map = serde_yaml::Mapping::new();
                for (k, v) in mapping.map.iter() {
                    new_map.insert(k.resolve(vars), v.resolve(vars));
                }
                serde_yaml::Value::Mapping(new_map)
            }
            Body::Tagged(tagged) => serde_yaml::Value::Tagged(Box::new(
                serde_yaml::value::TaggedValue {
                    tag: tagged.tag.clone(),
                    value: tagged.value.resolve(vars),
                },
            )),
            Body::Dynamic(dynamic) => dynamic.resolve(vars),
        }
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
                    res.push(DynamicValue::Var(Self::read_var(&mut chars)?));
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
    fn read_var(chars: &mut Peekable<Chars<'_>>) -> error::Result<String> {
        if chars.peek() != Some(&'{') {
            return Self::read_ident(chars);
        }

        _ = chars.next();
        let ident = Self::read_ident(chars)?;
        match chars.next() {
            Some('}') => Ok(ident),
            _ => Err(UrlError::UnclosedVar(ident).into()),
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
        }
    }
}

impl Eq for Body {}
