use std::collections::HashMap;

use crate::server::url::{token::UrlToken, var::UrlVar};

#[derive(Debug, Default)]
pub struct UrlSegment {
    pub tokens: Vec<UrlToken>,
}

impl UrlSegment {
    /// Creates new URL segment with given tokens
    pub fn new(tokens: Vec<UrlToken>) -> Self {
        Self { tokens }
    }

    /// Creates new empty URL segment
    pub fn empty() -> Self {
        Self { tokens: vec![] }
    }

    /// Pushes given value as static token only when not empty
    pub fn push_static(&mut self, value: String) {
        if !value.is_empty() {
            self.tokens.push(UrlToken::Static(value));
        }
    }

    /// Gets static URL string - only when tokens length is 1 with static token
    pub fn get_static(&self) -> Option<String> {
        if self.tokens.len() > 1 {
            return None;
        }
        match self.tokens.first() {
            Some(UrlToken::Static(s)) => Some(s.to_string()),
            _ => None,
        }
    }

    /// Checks if the URL segment matches the given URL.
    pub fn matches(
        &self,
        url: &str,
        vars: &mut HashMap<String, UrlVar>,
    ) -> bool {
        self.matches_inner(url, vars).is_some()
    }

    fn matches_inner(
        &self,
        url: &str,
        vars: &mut HashMap<String, UrlVar>,
    ) -> Option<()> {
        let mut chars = url.chars();
        let mut cur = chars.next();
        for (i, token) in self.tokens.iter().enumerate() {
            match token {
                UrlToken::Static(s) => {
                    self.match_static(s, &mut cur, &mut chars)?
                }
                UrlToken::Var { name, ty } => {
                    let val = match ty.as_str() {
                        "number" => self.match_number(&mut cur, &mut chars)?,
                        "string" => self.match_str(&mut cur, &mut chars, i)?,
                        _ => return None,
                    };
                    vars.insert(name.clone(), val);
                }
            }
        }

        Some(())
    }

    fn match_static(
        &self,
        exp: &str,
        cur: &mut Option<char>,
        chars: &mut dyn Iterator<Item = char>,
    ) -> Option<()> {
        for c in exp.chars() {
            if *cur != Some(c) {
                return None;
            }
            *cur = chars.next();
        }
        Some(())
    }

    /// Matches a number variable.
    fn match_number(
        &self,
        cur: &mut Option<char>,
        chars: &mut dyn Iterator<Item = char>,
    ) -> Option<UrlVar> {
        let mut val = 0;
        let mut found = false;

        while let Some(c) = cur
            && let Some(d) = c.to_digit(10)
        {
            found = true;
            val = val * 10 + d;
            *cur = chars.next();
        }

        found.then_some(UrlVar::Number(val))
    }

    /// Matches a string variables until it gets to a character from the next
    /// static token or end of url.
    fn match_str(
        &self,
        cur: &mut Option<char>,
        chars: &mut dyn Iterator<Item = char>,
        id: usize,
    ) -> Option<UrlVar> {
        let end = match self.tokens.get(id + 1) {
            Some(UrlToken::Static(s)) => s.chars().next().unwrap_or('/'),
            _ => return None,
        };

        let mut value = String::new();
        while let Some(c) = cur
            && c != &end
        {
            value.push(*c);
            *cur = chars.next();
        }

        (!value.is_empty()).then_some(UrlVar::String(value))
    }
}
