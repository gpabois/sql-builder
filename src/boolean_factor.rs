use sql_builder_macros::BooleanFactor;

use crate::grammar as G;
use crate::{Database, ToQuery};

#[derive(BooleanFactor)]
pub struct Not<BoolTest>(BoolTest)
where
    BoolTest: G::BooleanTest;

impl<DB, BoolTest> ToQuery<DB> for Not<BoolTest>
where
    DB: Database,
    BoolTest: G::BooleanTest + ToQuery<DB>,
{
    fn write<W: std::io::Write>(
        &self,
        stream: &mut W,
        ctx: &mut crate::ToQueryContext<DB>,
    ) -> Result<(), std::io::Error> {
        write!(stream, "NOT ")?;
        self.0.write(stream, ctx)
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
