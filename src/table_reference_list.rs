use sql_builder_macros::TableReferenceList;

use crate::grammar as G;
use crate::Database;
use crate::ToQuery;

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

impl<DB, Head, Tail> ToQuery<DB> for TableReferenceLink<Head, Tail>
where
    DB: Database,
    Head: G::TableReferenceList + ToQuery<DB>,
    Tail: G::TableReference + ToQuery<DB>,
{
    fn write<W: std::io::Write>(
        &self,
        stream: &mut W,
        ctx: &mut crate::ToQueryContext<DB>,
    ) -> Result<(), std::io::Error> {
        self.head.write(stream, ctx)?;
        write!(stream, ", ")?;
        self.tail.write(stream, ctx)
    }
}
