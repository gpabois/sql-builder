use crate::grammar as G;
use crate::Database;
use crate::ToQuery;
use sql_builder_macros::TableReferenceList;
use std::fmt::Write;

#[derive(TableReferenceList)]
pub struct TableReferenceLink<Head, Tail>
where
    Head: G::TableReferenceList,
    Tail: G::TableReference,
{
    pub(crate) head: Head,
    pub(crate) tail: Tail,
}

impl<Head, Tail> TableReferenceLink<Head, Tail>
where
    Head: G::TableReferenceList,
    Tail: G::TableReference,
{
    pub fn new(head: Head, tail: Tail) -> Self {
        Self { head, tail }
    }
}

impl<Head, Tail> ::std::fmt::Display for TableReferenceLink<Head, Tail>
where
    Head: G::TableReferenceList + std::fmt::Display,
    Tail: G::TableReference + std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}", self.head, self.tail)
    }
}

impl<'q, DB, Head, Tail> ToQuery<'q, DB> for TableReferenceLink<Head, Tail>
where
    DB: Database,
    Head: G::TableReferenceList + ToQuery<'q, DB>,
    Tail: G::TableReference + ToQuery<'q, DB>,
{
    fn write(&'q self, ctx: &mut crate::ToQueryContext<'q, DB>) -> std::fmt::Result {
        self.head.write(ctx)?;
        write!(ctx, ", ")?;
        self.tail.write(ctx)
    }
}
