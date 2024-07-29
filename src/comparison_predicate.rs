use sql_builder_macros::ComparisonPredicate;

use crate::{grammar as G, Database, ToQuery};

enum ComparisonKind {
    Equals,
    NotEquals,
    LessThan,
    GreaterThan,
    LessThanOrEquals,
    GreaterThanOrEquals,
}

impl AsRef<str> for ComparisonKind {
    fn as_ref(&self) -> &str {
        match self {
            ComparisonKind::Equals => "=",
            ComparisonKind::NotEquals => "<>",
            ComparisonKind::LessThan => "<",
            ComparisonKind::GreaterThan => ">",
            ComparisonKind::LessThanOrEquals => "<=",
            ComparisonKind::GreaterThanOrEquals => ">=",
        }
    }
}

impl<DB> ToQuery<DB> for ComparisonKind
where
    DB: Database,
{
    fn write<W: std::io::Write>(
        &self,
        stream: &mut W,
        _ctx: &mut crate::ToQueryContext<DB>,
    ) -> Result<(), std::io::Error> {
        write!(stream, "{}", self.as_ref())
    }
}

impl std::fmt::Display for ComparisonKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

#[derive(ComparisonPredicate)]
pub struct Compare<Lhs, Rhs>
where
    Lhs: G::RowValuePredicand,
    Rhs: G::RowValuePredicand,
{
    lhs: Lhs,
    rhs: Rhs,
    op: ComparisonKind,
}

impl<Lhs, Rhs> ::std::fmt::Display for Compare<Lhs, Rhs>
where
    Lhs: G::RowValuePredicand + ::std::fmt::Display,
    Rhs: G::RowValuePredicand + std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.lhs, self.op, self.rhs)
    }
}

impl<DB, Lhs, Rhs> ToQuery<DB> for Compare<Lhs, Rhs>
where
    DB: Database,
    Lhs: G::RowValuePredicand + ToQuery<DB>,
    Rhs: G::RowValuePredicand + ToQuery<DB>,
{
    fn write<W: std::io::Write>(
        &self,
        stream: &mut W,
        ctx: &mut crate::ToQueryContext<DB>,
    ) -> Result<(), std::io::Error> {
        self.lhs.write(stream, ctx)?;
        write!(stream, " ")?;
        self.op.write(stream, ctx)?;
        write!(stream, " ")?;
        self.rhs.write(stream, ctx)
    }
}

/// Checks if two values are equals
///
/// # SQL
/// ```sql
/// <lhs> = <rhs>
/// ```
pub fn eq<Lhs, Rhs>(lhs: Lhs, rhs: Rhs) -> Compare<Lhs, Rhs>
where
    Lhs: G::RowValuePredicand,
    Rhs: G::RowValuePredicand,
{
    Compare {
        lhs,
        rhs,
        op: ComparisonKind::Equals,
    }
}

#[inline]
/// Checks if two values are not equals
///
/// # SQL
/// ```sql
/// <lhs> <> <rhs>
/// ```
pub fn neq<Lhs, Rhs>(lhs: Lhs, rhs: Rhs) -> Compare<Lhs, Rhs>
where
    Lhs: G::RowValuePredicand,
    Rhs: G::RowValuePredicand,
{
    Compare {
        lhs,
        rhs,
        op: ComparisonKind::NotEquals,
    }
}

#[inline]
pub fn lt<Lhs, Rhs>(lhs: Lhs, rhs: Rhs) -> Compare<Lhs, Rhs>
where
    Lhs: G::RowValuePredicand,
    Rhs: G::RowValuePredicand,
{
    Compare {
        lhs,
        rhs,
        op: ComparisonKind::LessThan,
    }
}

#[inline]
pub fn lte<Lhs, Rhs>(lhs: Lhs, rhs: Rhs) -> Compare<Lhs, Rhs>
where
    Lhs: G::RowValuePredicand,
    Rhs: G::RowValuePredicand,
{
    Compare {
        lhs,
        rhs,
        op: ComparisonKind::LessThanOrEquals,
    }
}

#[inline]
pub fn gt<Lhs, Rhs>(lhs: Lhs, rhs: Rhs) -> Compare<Lhs, Rhs>
where
    Lhs: G::RowValuePredicand,
    Rhs: G::RowValuePredicand,
{
    Compare {
        lhs,
        rhs,
        op: ComparisonKind::GreaterThan,
    }
}

#[inline]
pub fn gte<Lhs, Rhs>(lhs: Lhs, rhs: Rhs) -> Compare<Lhs, Rhs>
where
    Lhs: G::RowValuePredicand,
    Rhs: G::RowValuePredicand,
{
    Compare {
        lhs,
        rhs,
        op: ComparisonKind::GreaterThanOrEquals,
    }
}
