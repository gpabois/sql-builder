use sql_builder_macros::BooleanTest;

use crate::{grammar as G, Database, ToQuery};

#[derive(BooleanTest)]
pub struct IsTruthValue<Primary, Truth>
where
    Primary: G::BooleanPrimary,
    Truth: G::TruthValue,
{
    lhs: Primary,
    rhs: Truth,
}

impl<Primary, Truth> ::std::fmt::Display for IsTruthValue<Primary, Truth>
where
    Primary: G::BooleanPrimary + std::fmt::Display,
    Truth: G::TruthValue + std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} IS {}", self.lhs, self.rhs)
    }
}

impl<DB, Primary, Truth> ToQuery<DB> for IsTruthValue<Primary, Truth>
where
    DB: Database,
    Primary: G::BooleanPrimary + ToQuery<DB>,
    Truth: G::TruthValue + ToQuery<DB>,
{
    fn write<W: std::io::Write>(
        &self,
        stream: &mut W,
        ctx: &mut crate::ToQueryContext<DB>,
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

impl<DB, Primary, Truth> ToQuery<DB> for IsNotTruthValue<Primary, Truth>
where
    DB: Database,
    Primary: G::BooleanPrimary + ToQuery<DB>,
    Truth: G::TruthValue + ToQuery<DB>,
{
    fn write<W: std::io::Write>(
        &self,
        stream: &mut W,
        ctx: &mut crate::ToQueryContext<DB>,
    ) -> Result<(), std::io::Error> {
        self.lhs.write(stream, ctx)?;
        write!(stream, " IS NOT ")?;
        self.rhs.write(stream, ctx)
    }
}

impl<Primary, Truth> ::std::fmt::Display for IsNotTruthValue<Primary, Truth>
where
    Primary: G::BooleanPrimary + std::fmt::Display,
    Truth: G::TruthValue + std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} IS {}", self.lhs, self.rhs)
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
