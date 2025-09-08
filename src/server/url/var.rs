#[derive(Debug, PartialEq, Eq, Clone)]
pub enum UrlVar {
    String(String),
    Number(u32),
}
