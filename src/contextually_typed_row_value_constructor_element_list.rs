use sql_builder_macros::ContextuallyTypedRowValueConstructorElementList;

use crate::{
    grammar::{
        ContextuallyTypedRowValueConstructorElement,
        ContextuallyTypedRowValueConstructorElementList,
    },
    ToQuery,
};

#[derive(ContextuallyTypedRowValueConstructorElementList)]
/// A linked-list of row values's constructor elements.
pub struct ContextuallyTypedRowValueConstructorElementLink<Head, Tail>(Head, Tail)
where
    Head: ContextuallyTypedRowValueConstructorElementList,
    Tail: ContextuallyTypedRowValueConstructorElement;

impl<Head, Tail> ToQuery for ContextuallyTypedRowValueConstructorElementLink<Head, Tail>
where
    Head: ContextuallyTypedRowValueConstructorElementList,
    Tail: ContextuallyTypedRowValueConstructorElement,
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
