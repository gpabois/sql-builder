use crate::grammar as G;

use crate::ToQuery;
use sql_builder_macros::SearchCondition;
use sqlx::Database;

#[derive(SearchCondition)]
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

impl<DB, SearchCond, BoolTerm> ToQuery<DB> for Or<SearchCond, BoolTerm>
where
    DB: Database,
    SearchCond: G::SearchCondition + ToQuery<DB>,
    BoolTerm: G::BooleanTerm + ToQuery<DB>,
{
    fn write<W: std::io::Write>(
        &self,
        stream: &mut W,
        ctx: &mut crate::ToQueryContext<DB>,
    ) -> Result<(), std::io::Error> {
        self.lhs.write(stream, ctx)?;
        write!(stream, " OR ")?;
        self.rhs.write(stream, ctx)
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
