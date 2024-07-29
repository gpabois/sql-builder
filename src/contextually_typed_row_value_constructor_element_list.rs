use sql_builder_macros::ContextuallyTypedRowValueConstructorElementList;

use crate::{grammar as G, Database, ToQuery};

#[derive(ContextuallyTypedRowValueConstructorElementList)]
/// A linked-list of row values's constructor elements.
pub struct ContextuallyTypedRowValueConstructorElementLink<Head, Tail>(Head, Tail)
where
    Head: G::ContextuallyTypedRowValueConstructorElementList,
    Tail: G::ContextuallyTypedRowValueConstructorElement;

impl<Head, Tail> ::std::fmt::Display for ContextuallyTypedRowValueConstructorElementLink<Head, Tail>
where
    Head: G::ContextuallyTypedRowValueConstructorElementList + std::fmt::Display,
    Tail: G::ContextuallyTypedRowValueConstructorElement + std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}", self.0, self.1)
    }
}

impl<DB, Head, Tail> ToQuery<DB> for ContextuallyTypedRowValueConstructorElementLink<Head, Tail>
where
    DB: Database,
    Head: G::ContextuallyTypedRowValueConstructorElementList + ToQuery<DB>,
    Tail: G::ContextuallyTypedRowValueConstructorElement + ToQuery<DB>,
{
    fn write<W: std::io::prelude::Write>(
        &self,
        stream: &mut W,
        ctx: &mut crate::ToQueryContext<DB>,
    ) -> Result<(), std::io::Error> {
        self.0.write(stream, ctx)?;
        write!(stream, ", ")?;
        self.1.write(stream, ctx)
    }
}
