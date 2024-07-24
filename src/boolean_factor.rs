use sql_builder_macros::BooleanFactor;

use crate::ToQuery;

use crate::helpers as H;
use crate::grammar as G;

#[derive(BooleanFactor)]
pub struct Not<BoolTest>(BoolTest)
where
    BoolTest: G::BooleanTest;

impl<Test> H::SelectSublist for Not<Test> where Test: G::BooleanTest {}
impl<Test> H::SearchCondition for Not<Test> where Test: G::BooleanTest {}

impl<BoolTest> ToQuery for Not<BoolTest>
where
    BoolTest: G::BooleanTest,
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
pub fn not(value: impl G::BooleanTest) -> impl G::BooleanFactor {
    Not(value)
}
