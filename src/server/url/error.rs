use thiserror::Error;

#[derive(Debug, Error)]
pub enum UrlError {
    #[error("unclosed variable `{0}` in URL pattern.")]
    UnclosedVar(String),
    #[error(
        "invalid character in variable {name}: expected {exp}, found {found}."
    )]
    UnexVarChar {
        name: String,
        exp: char,
        found: char,
    },
    #[error("identifier must start with a letter or _, found {0}.")]
    IdentStart(char),
    #[error("expected identifier")]
    MissingIdent,
    #[error("escape character `\\` must be followed by another character.")]
    EscapeCharMiss,
}

impl UrlError {
    pub fn unex_var_char(name: String, exp: char, found: char) -> Self {
        Self::UnexVarChar { name, exp, found }
    }
}
