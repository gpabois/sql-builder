//! A constructor to create a row value.

use sql_builder_macros::ContextuallyTypedRowValueConstructor;
use crate::grammar as G;

#[derive(ContextuallyTypedRowValueConstructor)]
pub struct RowValue<Elements>(Elements) 
    where Elements: G::ContextuallyTypedRowValueConstructorElementList;

impl<Elements> RowValue<Elements>
where Elements: G::ContextuallyTypedRowValueConstructorElementList {
    pub fn new(elements: Elements) -> Self {
        Self(elements)
    }
}