use std::fmt::Display;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum UrlVar {
    String(String),
    Number(u32),
}

impl Default for UrlVar {
    fn default() -> Self {
        Self::String(String::new())
    }
}

impl Display for UrlVar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UrlVar::String(s) => write!(f, "{s}"),
            UrlVar::Number(n) => write!(f, "{n}"),
        }
    }
}
