use sql_builder_macros::ContextuallyTypedRowValueConstructorElementList;

use crate::{grammar as G, Database, ToQuery};
use std::fmt::Write;

#[derive(ContextuallyTypedRowValueConstructorElementList)]
/// A linked-list of row values's constructor elements.
pub struct RowElementLink<Head, Tail>(Head, Tail)
where
    Head: G::ContextuallyTypedRowValueConstructorElementList,
    Tail: G::ContextuallyTypedRowValueConstructorElement;

impl<Head, Tail> RowElementLink<Head, Tail>
where
    Head: G::ContextuallyTypedRowValueConstructorElementList,
    Tail: G::ContextuallyTypedRowValueConstructorElement,
{
    pub fn new(head: Head, tail: Tail) -> Self {
        Self(head, tail)
    }
}

impl<Head, Tail> ::std::fmt::Display for RowElementLink<Head, Tail>
where
    Head: G::ContextuallyTypedRowValueConstructorElementList + std::fmt::Display,
    Tail: G::ContextuallyTypedRowValueConstructorElement + std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}", self.0, self.1)
    }
}

impl<'q, DB, Head, Tail> ToQuery<'q, DB> for RowElementLink<Head, Tail>
where
    DB: Database,
    Head: G::ContextuallyTypedRowValueConstructorElementList + ToQuery<'q, DB>,
    Tail: G::ContextuallyTypedRowValueConstructorElement + ToQuery<'q, DB>,
{
    fn write(&'q self, ctx: &mut crate::ToQueryContext<'q, DB>) -> ::std::fmt::Result {
        self.0.write(ctx)?;
        write!(ctx, ", ")?;
        self.1.write(ctx)
    }
}
