use sql_builder_macros::ContextuallyTypedRowValueExpressionList;

use crate::grammar as G;

use crate::ToQuery;

#[derive(ContextuallyTypedRowValueExpressionList)]
pub struct ContextuallyTypedRowExpressionLink<Head, Tail>
where
    Head: G::ContextuallyTypedRowValueExpressionList,
    Tail: G::ContextuallyTypedRowValueExpression
{
    head: Head,
    tail: Tail
}

impl<Head, Tail> ToQuery for ContextuallyTypedRowExpressionLink<Head, Tail>
where
    Head: G::ContextuallyTypedRowValueExpressionList,
    Tail: G::ContextuallyTypedRowValueExpression,
{
    fn write<W: std::io::prelude::Write>(
        &self,
        stream: &mut W,
        ctx: &mut crate::ToQueryContext,
    ) -> Result<(), std::io::Error> {
        self.0.write(stream, ctx)?;
        write!(stream, ", ")?;
        self.1.write(stream, ctx)
    }
}
