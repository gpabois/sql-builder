//! A constructor to create a row value.

use crate::{grammar as G, Database, ToQuery};
use sql_builder_macros::ContextuallyTypedRowValueConstructor;

#[derive(ContextuallyTypedRowValueConstructor)]
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

impl<DB, Elements> ToQuery<DB> for RowValue<Elements>
where
    DB: Database,
    Elements: G::ContextuallyTypedRowValueConstructorElementList + ToQuery<DB>,
{
    fn write<W: std::io::prelude::Write>(
        &self,
        stream: &mut W,
        ctx: &mut crate::ToQueryContext<DB>,
    ) -> Result<(), std::io::Error> {
        write!(stream, "(")?;
        self.0.write(stream, ctx)?;
        write!(stream, ")")
    }
}
