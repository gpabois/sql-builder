use crate::grammar as G;
use crate::{Database, ToQuery};
use sql_builder_macros::BooleanFactor;
use std::fmt::Write;

#[derive(BooleanFactor)]
pub struct Not<BoolTest>(BoolTest)
where
    BoolTest: G::BooleanTest;

impl<'q, DB, BoolTest> ToQuery<'q, DB> for Not<BoolTest>
where
    DB: Database,
    BoolTest: G::BooleanTest + ToQuery<'q, DB>,
{
    fn write(&'q self, ctx: &mut crate::ToQueryContext<'q, DB>) -> ::std::fmt::Result {
        write!(ctx, "NOT ")?;
        self.0.write(ctx)
    }
}

impl<BoolTest> ::std::fmt::Display for Not<BoolTest>
where
    BoolTest: G::BooleanTest + ::std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NOT {}", self.0)
    }
}

#[inline]
pub fn not(value: impl G::BooleanTest) -> impl G::BooleanFactor {
    Not(value)
}
