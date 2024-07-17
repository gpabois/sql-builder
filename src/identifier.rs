use regex::Regex;
use sql_builder_macros::Identifier;
use crate::{error::Error, ToQuery};

#[derive(Clone, Identifier)]
pub struct Identifier(String);

impl Identifier {
    pub fn is_valid(value: &str) -> bool {
        let re = Regex::new(r"^[A-Za-z_]([A-Za-z0-9_])*$").unwrap();
        re.is_match(value)
    }
}

impl TryFrom<&str> for Identifier {
    type Error = Error;
    
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if !Self::is_valid(value) {
            return Err(Error::invalid_identifier(value.to_owned()));
        }

        Ok(Self(value.to_owned()))
    }
}

impl ToQuery for Identifier {
    fn write<W: std::io::Write>(
        &self,
        stream: &mut W,
        ctx: &mut crate::ToQueryContext,
    ) -> Result<(), std::io::Error> {
        write!(stream, "{}", self.0)
    }
}

/// Creates an identifier.
/// 
/// Panics if ill-formatted.
pub fn id(value: &str) -> Identifier {
    Identifier::try_from(value).expect("cannot creates identifier")
}
