use crate::{grammar as G, Database, ToQuery};

use sql_builder_macros::ColumnNameList;

#[derive(ColumnNameList)]
pub struct ColumnNameLink<Head, Tail>
where
    Head: G::ColumnNameList,
    Tail: G::ColumnName,
{
    head: Head,
    tail: Tail,
}

impl<Head, Tail> std::fmt::Display for ColumnNameLink<Head, Tail>
where
    Head: G::ColumnNameList + std::fmt::Display,
    Tail: G::ColumnName + std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}", self.head, self.tail)
    }
}

impl<DB, Head, Tail> ToQuery<DB> for ColumnNameLink<Head, Tail>
where
    DB: Database,
    Head: G::ColumnNameList + ToQuery<DB>,
    Tail: G::ColumnName + ToQuery<DB>,
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

impl<Head, Tail> ColumnNameLink<Head, Tail>
where
    Head: G::ColumnNameList,
    Tail: G::ColumnName,
{
    pub fn new(head: Head, tail: Tail) -> Self {
        Self { head, tail }
    }
}
