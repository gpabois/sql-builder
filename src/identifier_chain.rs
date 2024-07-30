use crate::grammar as G;
use crate::Database;
use crate::ToQuery;
use sql_builder_macros::IdentifierChain;
use std::fmt::Write;

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

impl<'q, DB, Head, Tail> ToQuery<'q, DB> for IdentifierLink<Head, Tail>
where
    DB: Database,
    Head: G::IdentifierChain + ToQuery<'q, DB>,
    Tail: G::Identifier + ToQuery<'q, DB>,
{
    fn write(&'q self, ctx: &mut crate::ToQueryContext<'q, DB>) -> std::fmt::Result {
        self.0.write(ctx)?;
        write!(ctx, ".")?;
        self.1.write(ctx)
    }
}
