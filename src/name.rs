use regex::Regex;

use crate::{error::Error, ToQuery};

#[derive(Clone, PartialEq, Eq)]
pub enum Name {
    Quoted(String),
    Unquoted(String),
}

impl ToQuery for Name {
    fn write<W: std::io::prelude::Write>(
        &self,
        stream: &mut W,
        _ctx: &mut crate::ToQueryContext,
    ) -> Result<(), std::io::Error> {
        match self {
            Name::Quoted(value) => {
                write!(stream, "\"")?;
                write!(stream, "{}", value)?;
                write!(stream, "\"")
            }
            Name::Unquoted(value) => write!(stream, "{}", value),
        }
    }
}

impl TryFrom<&str> for Name {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

impl TryFrom<String> for Name {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::parse(&value)
    }
}

impl Name {
    pub fn parse(value: &str) -> Result<Self, Error> {
        if Self::is_valid_quoted_name(value) {
            Ok(Self::Quoted(value[1..value.len()].to_owned()))
        } else if Self::is_invalid_quoted_name(value) {
            Err(Error::invalid_quoted_name(value.to_owned()))
        } else if Self::is_valid_unquoted_name(value) {
            Ok(Self::Unquoted(value.to_owned()))
        } else {
            Err(Error::invalid_unquoted_name(value.to_owned()))
        }
    }

    #[inline]
    pub fn is_valid_quoted_name(value: &str) -> bool {
        value.starts_with('"') && value.ends_with('"')
    }

    #[inline]
    pub fn is_invalid_quoted_name(value: &str) -> bool {
        !Self::is_valid_quoted_name(value) && (value.starts_with('"') || value.ends_with('"'))
    }

    pub fn is_valid_unquoted_name(value: &str) -> bool {
        let re = Regex::new(r"^[A-Za-z_]([A-Za-z0-9_])*$").unwrap();
        re.is_match(value)
    }
}
