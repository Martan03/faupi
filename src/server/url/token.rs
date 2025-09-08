#[derive(Debug, PartialEq, Eq)]
pub enum UrlToken {
    Static(String),
    Var { name: String, ty: String },
}

impl UrlToken {
    pub fn var(name: String, ty: String) -> Self {
        Self::Var { name, ty }
    }

    pub fn string(name: String) -> Self {
        Self::var(name, "string".into())
    }
}
