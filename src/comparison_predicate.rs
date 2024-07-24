use sql_builder_macros::ComparisonPredicate;

use crate::grammar as G;
use crate::helpers as H;
use crate::ToQuery;

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

impl ToQuery for ComparisonKind {
    fn write<W: std::io::Write>(
        &self,
        stream: &mut W,
        _ctx: &mut crate::ToQueryContext,
    ) -> Result<(), std::io::Error> {
        write!(stream, "{}", self.as_ref())
    }
}

#[derive(ComparisonPredicate)]
pub struct Compare<Lhs, Rhs>
where
    Lhs: G::RowValueConstructor,
    Rhs: G::RowValueConstructor,
{
    lhs: Lhs,
    rhs: Rhs,
    op: ComparisonKind,
}

impl<Lhs, Rhs> H::ValueExpression for Compare<Lhs, Rhs>
where
    Lhs: G::RowValueConstructor,
    Rhs: G::RowValueConstructor,
{
}

impl<Lhs, Rhs> H::SelectSublist for Compare<Lhs, Rhs>
where
    Lhs: G::RowValueConstructor,
    Rhs: G::RowValueConstructor,
{
}

impl<Lhs, Rhs> H::SearchCondition for Compare<Lhs, Rhs>
where
    Lhs: G::RowValueConstructor,
    Rhs: G::RowValueConstructor,
{
}

impl<Lhs, Rhs> ToQuery for Compare<Lhs, Rhs>
where
    Lhs: G::RowValueConstructor,
    Rhs: G::RowValueConstructor,
{
    fn write<W: std::io::Write>(
        &self,
        stream: &mut W,
        ctx: &mut crate::ToQueryContext,
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
pub fn eq<Lhs, Rhs>(lhs: Lhs, rhs: Rhs) -> impl G::ComparisonPredicate
where
    Lhs: G::RowValueConstructor,
    Rhs: G::RowValueConstructor,
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
pub fn neq(
    lhs: impl G::RowValueConstructor,
    rhs: impl G::RowValueConstructor,
) -> impl G::ComparisonPredicate {
    Compare {
        lhs,
        rhs,
        op: ComparisonKind::NotEquals,
    }
}

#[inline]
pub fn lt(
    lhs: impl G::RowValueConstructor,
    rhs: impl G::RowValueConstructor,
) -> impl G::ComparisonPredicate {
    Compare {
        lhs,
        rhs,
        op: ComparisonKind::LessThan,
    }
}

#[inline]
pub fn lte(
    lhs: impl G::RowValueConstructor,
    rhs: impl G::RowValueConstructor,
) -> impl G::ComparisonPredicate {
    Compare {
        lhs,
        rhs,
        op: ComparisonKind::LessThanOrEquals,
    }
}

#[inline]
pub fn gt(
    lhs: impl G::RowValueConstructor,
    rhs: impl G::RowValueConstructor,
) -> impl G::ComparisonPredicate {
    Compare {
        lhs,
        rhs,
        op: ComparisonKind::GreaterThan,
    }
}

#[inline]
pub fn gte(
    lhs: impl G::RowValueConstructor,
    rhs: impl G::RowValueConstructor,
) -> impl G::ComparisonPredicate {
    Compare {
        lhs,
        rhs,
        op: ComparisonKind::GreaterThanOrEquals,
    }
}

#[cfg(test)]
mod tests {
    use crate::{eq, gt, gte, id, lit, lt, lte, neq, ToQuery};

    #[test]
    fn test_eq() {
        let cmp = eq(id("test"), lit(10.123f32));
        let sql = cmp.to_raw_query().unwrap();
        assert_eq!(sql, "test = 10.123");
    }

    #[test]
    fn test_neq() {
        let cmp = neq(id("test"), lit(10.123f32));
        let sql = cmp.to_raw_query().unwrap();
        assert_eq!(sql, "test <> 10.123");
    }

    #[test]
    fn test_lt() {
        let cmp = lt(id("test"), lit(10.123f32));
        let sql = cmp.to_raw_query().unwrap();
        assert_eq!(sql, "test < 10.123");
    }

    #[test]
    fn test_lte() {
        let cmp = lte(id("test"), lit(10.123f32));
        let sql = cmp.to_raw_query().unwrap();
        assert_eq!(sql, "test <= 10.123");
    }

    #[test]
    fn test_gt() {
        let cmp = gt(id("test"), lit(10.123f32));
        let sql = cmp.to_raw_query().unwrap();
        assert_eq!(sql, "test > 10.123");
    }

    #[test]
    fn test_gte() {
        let cmp = gte(id("test"), lit(10.123f32));
        let sql = cmp.to_raw_query().unwrap();
        assert_eq!(sql, "test >= 10.123");
    }
}
