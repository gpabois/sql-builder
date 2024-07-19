use sql_builder_macros::BooleanFactor;

use crate::{grammar::{self, BooleanFactor, BooleanTest}, ToQuery};

#[derive(BooleanFactor)]
pub struct Not<BoolTest>(pub(crate) BoolTest)
where
    BoolTest: grammar::BooleanTest;

impl<BoolTest> ToQuery for Not<BoolTest>
where
    BoolTest: grammar::BooleanTest,
{
    fn write<W: std::io::Write>(
        &self,
        stream: &mut W,
        ctx: &mut crate::ToQueryContext,
    ) -> Result<(), std::io::Error> {
        write!(stream, "NOT ")?;
        self.0.write(stream, ctx)
    }
}

#[inline]
pub fn not(value: impl BooleanTest) -> impl BooleanFactor {
    Not(value)
}
