use crate::grammar as G;
use crate::helpers as H;
use crate::{
    error::Error, identifier_chain::IdentifierChain, table_reference_list::TableReferenceLink,
    ToQuery,
};
use regex::Regex;
use sql_builder_macros::Identifier;

#[derive(Clone, Identifier)]
pub struct IdentifierRef<'s>(&'s str);

impl H::ColumnNameList for IdentifierRef<'_> {}
impl H::ValueExpression for IdentifierRef<'_> {}
impl H::SelectSublist for IdentifierRef<'_> {}
impl H::TableReference for IdentifierRef<'_> {}
impl H::TableReferenceList for IdentifierRef<'_> {}

impl H::IdentifierChain for IdentifierRef<'_> {
    fn add_identifier(self, id: impl G::Identifier) -> impl G::IdentifierChain {
        IdentifierChain::new(self, id)
    }
}

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

impl ToQuery for IdentifierRef<'_> {
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
pub fn id(value: &str) -> IdentifierRef<'_> {
    IdentifierRef::try_from(value).expect("cannot creates identifier")
}
