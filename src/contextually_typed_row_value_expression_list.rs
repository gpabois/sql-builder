use sql_builder_macros::ContextuallyTypedRowValueExpressionList;

use crate::grammar as G;
use crate::Database;
use crate::ToQuery;

#[derive(ContextuallyTypedRowValueExpressionList)]
pub struct ContextuallyTypedRowExpressionLink<Head, Tail>
where
    Head: G::ContextuallyTypedRowValueExpressionList,
    Tail: G::ContextuallyTypedRowValueExpression,
{
    head: Head,
    tail: Tail,
}

impl<Head, Tail> ContextuallyTypedRowExpressionLink<Head, Tail>
where
    Head: G::ContextuallyTypedRowValueExpressionList,
    Tail: G::ContextuallyTypedRowValueExpression,
{
    pub fn new(head: Head, tail: Tail) -> Self {
        Self { head, tail }
    }
}
impl<Head, Tail> std::fmt::Display for ContextuallyTypedRowExpressionLink<Head, Tail>
where
    Head: G::ContextuallyTypedRowValueExpressionList + std::fmt::Display,
    Tail: G::ContextuallyTypedRowValueExpression + std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}", self.head, self.tail)
    }
}
impl<DB, Head, Tail> ToQuery<DB> for ContextuallyTypedRowExpressionLink<Head, Tail>
where
    DB: Database,
    Head: G::ContextuallyTypedRowValueExpressionList + ToQuery<DB>,
    Tail: G::ContextuallyTypedRowValueExpression + ToQuery<DB>,
{
    fn write<W: std::io::prelude::Write>(
        &self,
        stream: &mut W,
        ctx: &mut crate::ToQueryContext<DB>,
    ) -> Result<(), std::io::Error> {
        self.head.write(stream, ctx)?;
        write!(stream, ", ")?;
        self.tail.write(stream, ctx)
    }
}
