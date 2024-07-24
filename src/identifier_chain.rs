use sql_builder_macros::IdentifierChain;

use crate::grammar as G;
use crate::helpers as H;
use crate::ToQuery;

#[derive(IdentifierChain)]
pub struct IdentifierChain<Head, Tail>(Head, Tail)
where
    Head: G::IdentifierChain,
    Tail: G::Identifier;


impl<Head, Tail> IdentifierChain<Head, Tail>
where
    Head: G::IdentifierChain,
    Tail: G::Identifier,
{
    pub fn new(head: Head, tail: Tail) -> Self {
        Self(head, tail)
    }
}

impl<Head, Tail> H::SelectSublist for IdentifierChain<Head, Tail>
where
    Head: G::IdentifierChain,
    Tail: G::Identifier
{}

impl<Head, Tail> H::IdentifierChain for IdentifierChain<Head, Tail>
where
    Head: G::IdentifierChain,
    Tail: G::Identifier,
{
    fn add_identifier(self, id: impl G::Identifier) -> impl G::IdentifierChain {
        IdentifierChain(self, id)
    }
}

impl<Head, Tail> ToQuery for IdentifierChain<Head, Tail>
where
    Head: G::IdentifierChain,
    Tail: G::Identifier,
{
    fn write<W: std::io::prelude::Write>(
        &self,
        stream: &mut W,
        ctx: &mut crate::ToQueryContext,
    ) -> Result<(), std::io::Error> {
        todo!()
    }
}

