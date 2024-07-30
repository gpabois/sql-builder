use crate::{grammar as G, Database, ToQuery};
use sql_builder_macros::BooleanTest;
use std::fmt::Write;

#[derive(Clone, Copy, BooleanTest)]
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

impl<'q, DB, Primary, Truth> ToQuery<'q, DB> for IsTruthValue<Primary, Truth>
where
    DB: Database,
    Primary: G::BooleanPrimary + ToQuery<'q, DB>,
    Truth: G::TruthValue + ToQuery<'q, DB>,
{
    fn write(&'q self, ctx: &mut crate::ToQueryContext<'q, DB>) -> ::std::fmt::Result {
        self.lhs.write(ctx)?;
        write!(ctx, " IS ")?;
        self.rhs.write(ctx)
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

impl<'q, DB, Primary, Truth> ToQuery<'q, DB> for IsNotTruthValue<Primary, Truth>
where
    DB: Database,
    Primary: G::BooleanPrimary + ToQuery<'q, DB>,
    Truth: G::TruthValue + ToQuery<'q, DB>,
{
    fn write(&'q self, ctx: &mut crate::ToQueryContext<'q, DB>) -> ::std::fmt::Result {
        self.lhs.write(ctx)?;
        write!(ctx, " IS NOT ")?;
        self.rhs.write(ctx)
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
