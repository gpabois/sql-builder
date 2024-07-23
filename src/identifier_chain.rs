use sql_builder_macros::IdentifierChain;

use crate::grammar as G;
use crate::helpers as H;

#[derive(IdentifierChain)]
pub struct IdentifierChain<Head, Tail>(Head, Tail)
where Head: G::IdentifierChain, Tail: G::Identifier;

impl<Head, Tail> H::IdentifierChain for IdentifierChain<Head, Tail>
where Head: G::IdentifierChain, Tail: G::Identifier {
    fn add_identifier(self, id: impl G::Identifier) -> impl G::IdentifierChain {
        IdentifierChain(self, id)
    }
}