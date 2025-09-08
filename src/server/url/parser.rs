use crate::{
    error::Result,
    server::url::{error::UrlError, segment::UrlSegment, token::UrlToken},
};

pub struct UrlParser<'a> {
    chars: &'a mut dyn Iterator<Item = char>,
    cur: Option<char>,
}

impl<'a> UrlParser<'a> {
    /// Creates new URL parser
    pub fn new(url: &'a mut dyn Iterator<Item = char>) -> Self {
        let cur = url.next();
        Self { chars: url, cur }
    }

    /// Gets next URL segment.
    pub fn next(&mut self) -> Result<Option<UrlSegment>> {
        if self.cur.is_none() {
            return Ok(None);
        }

        let mut segment = UrlSegment::empty();
        let mut seg = String::new();
        while let Some(mut c) = self.cur {
            if c == '{' {
                segment.push_static(seg);
                seg = String::new();

                segment.tokens.push(self.read_var()?);
                self.cur = self.chars.next();
                continue;
            }

            if c == '/' {
                self.cur = self.chars.next();
                break;
            }

            if c == '\\' {
                c = self.chars.next().ok_or(UrlError::EscapeCharMiss)?;
            }
            seg.push(c);
            self.cur = self.chars.next();
        }
        segment.push_static(seg);
        Ok(Some(segment))
    }

    /// Reads URL variable, defaults to string when type not provided.
    /// `{ name }` - string variable, `{ name: type }` - variable with set type.
    fn read_var(&mut self) -> Result<UrlToken> {
        let name = self.read_ident()?;
        self.skip_whitespace();
        if self.cur == Some('}') {
            return Ok(UrlToken::string(name));
        }

        match self.cur {
            Some(c) if c != ':' => {
                return Err(UrlError::unex_var_char(name, ':', c).into());
            }
            None => return Err(UrlError::UnclosedVar(name).into()),
            _ => {}
        }

        let ty = self.read_ident()?;
        self.skip_whitespace();
        if self.cur != Some('}') {
            return Err(UrlError::UnclosedVar(name).into());
        }

        return Ok(UrlToken::var(name, ty));
    }

    /// Reads identifier, where identifier can start with any alphabetic
    /// character or underscore + can contain numerics.
    fn read_ident(&mut self) -> Result<String> {
        self.skip_whitespace();

        let mut ident = String::new();
        self.cur = self.chars.next();
        match self.cur {
            Some(c) if c.is_ascii_alphabetic() || c == '_' => {
                ident.push(c);
                self.cur = self.chars.next();
            }
            Some(c) => return Err(UrlError::IdentStart(c).into()),
            _ => return Err(UrlError::MissingIdent.into()),
        }

        while let Some(c) = self.cur {
            if !c.is_ascii_alphanumeric() && c != '_' {
                break;
            }
            ident.push(c);
            self.cur = self.chars.next();
        }
        return Ok(ident);
    }

    /// Reads leading whitespaces.
    fn skip_whitespace(&mut self) {
        while let Some(c) = self.cur
            && c.is_whitespace()
        {
            self.cur = self.chars.next();
        }
    }
}
