use sql_builder_macros::SelectSublist;

use crate::grammar as G;
use crate::Database;
use crate::ToQuery;

/// A list of select expressions.
/// Work recursively.
#[derive(SelectSublist)]
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

impl<DB, Head, Tail> ToQuery<DB> for SelectLink<Head, Tail>
where
    DB: Database,
    Head: G::SelectSublist + ToQuery<DB>,
    Tail: G::SelectSublistElement + ToQuery<DB>,
{
    fn write<W: std::io::Write>(
        &self,
        stream: &mut W,
        ctx: &mut crate::ToQueryContext<DB>,
    ) -> Result<(), std::io::Error> {
        self.0.write(stream, ctx)?;
        write!(stream, ", ")?;
        self.1.write(stream, ctx)
    }
}

