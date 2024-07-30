use crate::grammar as G;

use crate::ToQuery;
use sql_builder_macros::SearchCondition;
use sqlx::Database;
use std::fmt::Write;

#[derive(Clone, Copy, SearchCondition)]
pub struct Or<Lhs, Rhs>
where
    Lhs: G::SearchCondition,
    Rhs: G::BooleanTerm,
{
    lhs: Lhs,
    rhs: Rhs,
}

impl<SearchCond, BoolTerm> ::std::fmt::Display for Or<SearchCond, BoolTerm>
where
    SearchCond: G::SearchCondition + std::fmt::Display,
    BoolTerm: G::BooleanTerm + std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} OR {}", self.lhs, self.rhs)
    }
}

impl<'q, DB, SearchCond, BoolTerm> ToQuery<'q, DB> for Or<SearchCond, BoolTerm>
where
    DB: Database,
    SearchCond: G::SearchCondition + ToQuery<'q, DB>,
    BoolTerm: G::BooleanTerm + ToQuery<'q, DB>,
{
    fn write(&'q self, ctx: &mut crate::ToQueryContext<'q, DB>) -> std::fmt::Result {
        self.lhs.write(ctx)?;
        write!(ctx, " OR ")?;
        self.rhs.write(ctx)
    }
}

#[inline]
/// Creates an OR search condition
pub fn or<Lhs, Rhs>(lhs: Lhs, rhs: Rhs) -> Or<Lhs, Rhs>
where
    Lhs: G::SearchCondition,
    Rhs: G::BooleanTerm,
{
    Or { lhs, rhs }
}
