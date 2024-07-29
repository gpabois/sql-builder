use crate::{error::Error, Database, ToQuery};
use regex::Regex;
use sql_builder_macros::Identifier;

#[derive(Clone, Identifier)]
pub struct IdentifierRef<'s>(&'s str);

impl IdentifierRef<'_> {
    pub fn is_valid(value: &str) -> bool {
        let re = Regex::new(r"^[A-Za-z_]([A-Za-z0-9_])*$").unwrap();
        re.is_match(value)
    }
}

impl<'s> TryFrom<&'s str> for IdentifierRef<'s> {
    type Error = Error;

    fn try_from(value: &'s str) -> Result<Self, Self::Error> {
        if !Self::is_valid(value) {
            return Err(Error::invalid_identifier(value.to_owned()));
        }

        Ok(Self(value))
    }
}

impl<DB> ToQuery<DB> for IdentifierRef<'_>
where
    DB: Database,
{
    fn write<W: std::io::Write>(
        &self,
        stream: &mut W,
        _ctx: &mut crate::ToQueryContext<DB>,
    ) -> Result<(), std::io::Error> {
        write!(stream, "{}", self.0)
    }
}

impl std::fmt::Display for IdentifierRef<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

/// Creates an identifier.
///
/// Panics if ill-formatted.
pub fn id(value: &str) -> IdentifierRef<'_> {
    IdentifierRef::try_from(value).expect("cannot creates identifier")
}
