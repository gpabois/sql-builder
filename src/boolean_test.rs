use sql_builder_macros::BooleanTest;

use crate::ToQuery;

use crate::grammar as G;
use crate::helpers as H;

#[derive(BooleanTest)]
pub struct IsTruthValue<Primary, Truth>
where
    Primary: G::BooleanPrimary,
    Truth: G::TruthValue,
{
    lhs: Primary,
    rhs: Truth,
}

impl<Primary, Truth> H::ValueExpression for IsTruthValue<Primary, Truth>
where
    Primary: G::BooleanPrimary,
    Truth: G::TruthValue,
{
}

impl<Primary, Truth> H::SearchCondition for IsTruthValue<Primary, Truth>
where
    Primary: G::BooleanPrimary,
    Truth: G::TruthValue,
{
}

impl<Primary, Truth> H::SelectSublist for IsTruthValue<Primary, Truth>
where
    Primary: G::BooleanPrimary,
    Truth: G::TruthValue,
{
}

impl<Primary, Truth> ToQuery for IsTruthValue<Primary, Truth>
where
    Primary: G::BooleanPrimary,
    Truth: G::TruthValue,
{
    fn write<W: std::io::Write>(
        &self,
        stream: &mut W,
        ctx: &mut crate::ToQueryContext,
    ) -> Result<(), std::io::Error> {
        self.lhs.write(stream, ctx)?;
        write!(stream, " IS ")?;
        self.rhs.write(stream, ctx)
    }
}

#[derive(BooleanTest)]
pub struct IsNotTruthValue<Primary, Truth>
where
    Primary: G::BooleanPrimary,
    Truth: G::TruthValue,
{
    lhs: Primary,
    rhs: Truth,
}

impl<Primary, Truth> H::ValueExpression for IsNotTruthValue<Primary, Truth>
where
    Primary: G::BooleanPrimary,
    Truth: G::TruthValue,
{
}

impl<Primary, Truth> H::SelectSublist for IsNotTruthValue<Primary, Truth>
where
    Primary: G::BooleanPrimary,
    Truth: G::TruthValue,
{
}

impl<Primary, Truth> H::SearchCondition for IsNotTruthValue<Primary, Truth>
where
    Primary: G::BooleanPrimary,
    Truth: G::TruthValue,
{
}

impl<Primary, Truth> ToQuery for IsNotTruthValue<Primary, Truth>
where
    Primary: G::BooleanPrimary,
    Truth: G::TruthValue,
{
    fn write<W: std::io::Write>(
        &self,
        stream: &mut W,
        ctx: &mut crate::ToQueryContext,
    ) -> Result<(), std::io::Error> {
        self.lhs.write(stream, ctx)?;
        write!(stream, " IS NOT ")?;
        self.rhs.write(stream, ctx)
    }
}

#[inline]
pub fn is_truth_value(lhs: impl G::BooleanPrimary, rhs: impl G::TruthValue) -> impl G::BooleanTest {
    IsTruthValue { lhs, rhs }
}

#[inline]
pub fn is_not_truth_value(
    lhs: impl G::BooleanPrimary,
    rhs: impl G::TruthValue,
) -> impl G::BooleanTest {
    IsNotTruthValue { lhs, rhs }
}
