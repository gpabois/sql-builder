use sql_builder_macros::BooleanTest;

use crate::{grammar::{self, BooleanPrimary, BooleanTest, TruthValue}, ToQuery};

#[derive(BooleanTest)]
pub struct IsTruthValue<BoolPrimary, TruthValue>
where
    BoolPrimary: grammar::BooleanPrimary,
    TruthValue: grammar::TruthValue,
{
    pub(crate) lhs: BoolPrimary,
    pub(crate) rhs: TruthValue,
}

impl<BoolPrimary, TruthValue> ToQuery for IsTruthValue<BoolPrimary, TruthValue>
where
    BoolPrimary: grammar::BooleanPrimary,
    TruthValue: grammar::TruthValue,
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
pub struct IsNotTruthValue<BoolPrimary, TruthValue>
where
    BoolPrimary: grammar::BooleanPrimary,
    TruthValue: grammar::TruthValue,
{
    pub(crate) lhs: BoolPrimary,
    pub(crate) rhs: TruthValue,
}

impl<BoolPrimary, TruthValue> ToQuery for IsNotTruthValue<BoolPrimary, TruthValue>
where
    BoolPrimary: grammar::BooleanPrimary,
    TruthValue: grammar::TruthValue,
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
pub fn is_truth_value(lhs: impl BooleanPrimary, rhs: impl TruthValue) -> impl BooleanTest {
    IsTruthValue{lhs, rhs}
}

#[inline]
pub fn is_not_truth_value(lhs: impl BooleanPrimary, rhs: impl TruthValue) -> impl BooleanTest {
    IsNotTruthValue{lhs, rhs}
}
