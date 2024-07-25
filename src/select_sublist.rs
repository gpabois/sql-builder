use sql_builder_macros::SelectSublist;

use crate::blank::Blank;
use crate::ToQuery;
use crate::grammar as G;
use crate::helpers as H;

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
    Tail: G::SelectSublistElement
{
    pub fn new(head: Head, tail: Tail) -> Self {
        Self(head, tail)
    }
}

impl<Head, Tail> ToQuery for SelectLink<Head, Tail>
where
    Head: G::SelectSublist,
    Tail: G::SelectSublistElement,
{
    fn write<W: std::io::Write>(
        &self,
        stream: &mut W,
        ctx: &mut crate::ToQueryContext,
    ) -> Result<(), std::io::Error> {
        self.0.write(stream, ctx)?;
        write!(stream, ", ")?;
        self.1.write(stream, ctx)
    }
}