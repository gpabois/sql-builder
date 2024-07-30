//! A constructor to create a row value.
use crate::{grammar as G, Database, ToQuery};
use sql_builder_macros::ContextuallyTypedRowValueConstructor;
use std::fmt::Write;

#[derive(Clone, Copy, ContextuallyTypedRowValueConstructor)]
pub struct RowValue<Elements>(Elements)
where
    Elements: G::ContextuallyTypedRowValueConstructorElementList;

impl<Elements> RowValue<Elements>
where
    Elements: G::ContextuallyTypedRowValueConstructorElementList,
{
    pub fn new(elements: Elements) -> Self {
        Self(elements)
    }
}

impl<Elements> ::std::fmt::Display for RowValue<Elements>
where
    Elements: G::ContextuallyTypedRowValueConstructorElementList + std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({})", self.0)
    }
}

impl<'q, DB, Elements> ToQuery<'q, DB> for RowValue<Elements>
where
    DB: Database,
    Elements: G::ContextuallyTypedRowValueConstructorElementList + ToQuery<'q, DB>,
{
    fn write(&'q self, ctx: &mut crate::ToQueryContext<'q, DB>) -> std::fmt::Result {
        write!(ctx, "(")?;
        self.0.write(ctx)?;
        write!(ctx, ")")
    }
}
