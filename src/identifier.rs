use crate::{error::Error, identifier_chain::IdentifierChain, table_reference_list::TableReferenceLink, ToQuery};
use regex::Regex;
use crate::helpers as H;
use crate::grammar as G;
use sql_builder_macros::Identifier;

#[derive(Clone, Identifier)]
pub struct Identifier(String);

impl H::SelectSublist for Identifier {}
impl H::TableReference for Identifier {}
impl H::TableReferenceList for Identifier {
    fn add_table_reference(
        self,
        table_ref: impl G::TableReference,
    ) -> impl G::TableReferenceList {
        TableReferenceLink::new(self, table_ref)
    }
}

impl H::IdentifierChain for Identifier {
    fn add_identifier(self, id: impl G::Identifier) -> impl G::IdentifierChain {
        IdentifierChain::new(self, id)
    }
}

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
        _ctx: &mut crate::ToQueryContext,
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
