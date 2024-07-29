use sql_builder_macros::IdentifierChain;

use crate::grammar as G;
use crate::Database;
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

impl<Head, Tail> ::std::fmt::Display for IdentifierLink<Head, Tail>
where
    Head: G::IdentifierChain + std::fmt::Display,
    Tail: G::Identifier + std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}", self.0, self.1)
    }
}

impl<DB, Head, Tail> ToQuery<DB> for IdentifierLink<Head, Tail>
where
    DB: Database,
    Head: G::IdentifierChain + ToQuery<DB>,
    Tail: G::Identifier + ToQuery<DB>,
{
    fn write<W: std::io::prelude::Write>(
        &self,
        stream: &mut W,
        ctx: &mut crate::ToQueryContext<DB>,
    ) -> Result<(), std::io::Error> {
        self.0.write(stream, ctx)?;
        write!(stream, ".")?;
        self.1.write(stream, ctx)
    }
}
