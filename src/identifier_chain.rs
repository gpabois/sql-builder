use sql_builder_macros::IdentifierChain;

use crate::grammar as G;
use crate::ToQuery;

#[derive(IdentifierChain)]
pub struct IdentifierLink<Head, Tail>(Head, Tail)
where
    Head: G::IdentifierChain,
    Tail: G::Identifier;

impl<Head, Tail> IdentifierLink<Head, Tail>
where
    Head: G::IdentifierChain,
    Tail: G::Identifier,
{
    pub fn new(head: Head, tail: Tail) -> Self {
        Self(head, tail)
    }
}

impl<Head, Tail> ToQuery for IdentifierLink<Head, Tail>
where
    Head: G::IdentifierChain,
    Tail: G::Identifier,
{
    fn write<W: std::io::prelude::Write>(
        &self,
        stream: &mut W,
        ctx: &mut crate::ToQueryContext,
    ) -> Result<(), std::io::Error> {
        self.0.write(stream, ctx)?;
        write!(stream, ".")?;
        self.1.write(stream, ctx)
    }
}
