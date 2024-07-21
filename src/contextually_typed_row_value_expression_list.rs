use sql_builder_macros::ContextuallyTypedRowValueExpressionList;

use crate::{
    grammar::{ContextuallyTypedRowValueExpression, ContextuallyTypedRowValueExpressionList},
    ToQuery,
};

#[derive(ContextuallyTypedRowValueExpressionList)]
pub struct ContextuallyTypedRowExpressionLink<Head, Tail>(pub(crate) Head, pub(crate) Tail)
where
    Head: ContextuallyTypedRowValueExpressionList,
    Tail: ContextuallyTypedRowValueExpression;

impl<Head, Tail> ToQuery for ContextuallyTypedRowExpressionLink<Head, Tail>
where
    Head: ContextuallyTypedRowValueExpressionList,
    Tail: ContextuallyTypedRowValueExpression,
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
