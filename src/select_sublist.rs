use crate::grammar as G;
use crate::Database;
use crate::ToQuery;
use sql_builder_macros::SelectSublist;
use std::fmt::Write;

#[derive(Clone, Copy, SelectSublist)]
/// A list of select expressions.
/// Work recursively.
pub struct SelectLink<Head, Tail>(Head, Tail)
where
    Head: G::SelectSublist,
    Tail: G::SelectSublistElement;

impl<Head, Tail> SelectLink<Head, Tail>
where
    Head: G::SelectSublist,
    Tail: G::SelectSublistElement,
{
    pub fn new(head: Head, tail: Tail) -> Self {
        Self(head, tail)
    }
}

impl<Head, Tail> ::std::fmt::Display for SelectLink<Head, Tail>
where
    Head: G::SelectSublist + ::std::fmt::Display,
    Tail: G::SelectSublistElement + ::std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}", self.0, self.1)
    }
}

impl<'q, DB, Head, Tail> ToQuery<'q, DB> for SelectLink<Head, Tail>
where
    DB: Database,
    Head: G::SelectSublist + ToQuery<'q, DB>,
    Tail: G::SelectSublistElement + ToQuery<'q, DB>,
{
    fn write(&'q self, ctx: &mut crate::ToQueryContext<'q, DB>) -> std::fmt::Result {
        self.0.write(ctx)?;
        write!(ctx, ", ")?;
        self.1.write(ctx)
    }
}
