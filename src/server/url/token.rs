use crate::{error::Result, server::url::error::UrlError};

#[derive(Debug, PartialEq, Eq)]
pub enum UrlToken {
    Static(String),
    Var { name: String, ty: String },
}

impl UrlToken {
    pub fn var(name: String, ty: String) -> Result<Self> {
        if !["string", "number"].contains(&ty.as_str()) {
            return Err(UrlError::InvalidType(ty).into());
        }
        Ok(Self::Var { name, ty })
    }

    pub fn string(name: String) -> Self {
        Self::Var {
            name,
            ty: "string".into(),
        }
    }
}
