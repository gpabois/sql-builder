use crate::grammar as G;
use crate::Database;
use crate::ToQuery;
use sql_builder_macros::ContextuallyTypedRowValueExpressionList;
use std::fmt::Write;

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
impl<'q, DB, Head, Tail> ToQuery<'q, DB> for ContextuallyTypedRowExpressionLink<Head, Tail>
where
    DB: Database,
    Head: G::ContextuallyTypedRowValueExpressionList + ToQuery<'q, DB>,
    Tail: G::ContextuallyTypedRowValueExpression + ToQuery<'q, DB>,
{
    fn write(&'q self, ctx: &mut crate::ToQueryContext<'q, DB>) -> ::std::fmt::Result {
        self.head.write(ctx)?;
        write!(ctx, ", ")?;
        self.tail.write(ctx)
    }
}
