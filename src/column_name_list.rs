use crate::{grammar as G, Database, ToQuery};
use sql_builder_macros::ColumnNameList;
use std::fmt::Write;

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

impl<'q, DB, Head, Tail> ToQuery<'q, DB> for ColumnNameLink<Head, Tail>
where
    DB: Database,
    Head: G::ColumnNameList + ToQuery<'q, DB>,
    Tail: G::ColumnName + ToQuery<'q, DB>,
{
    fn write(&'q self, ctx: &mut crate::ToQueryContext<'q, DB>) -> ::std::fmt::Result {
        self.head.write(ctx)?;
        write!(ctx, ", ")?;
        self.tail.write(ctx)
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
