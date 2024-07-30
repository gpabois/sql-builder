use crate::{grammar as G, Database, ToQuery};
use sql_builder_macros::BooleanTerm;
use std::fmt::Write;

#[derive(Clone, Copy, BooleanTerm)]
pub struct And<Lhs, Rhs>
where
    Lhs: G::BooleanTerm,
    Rhs: G::BooleanFactor,
{
    lhs: Lhs,
    rhs: Rhs,
}

impl<Lhs, Rhs> ::std::fmt::Display for And<Lhs, Rhs>
where
    Lhs: G::BooleanTerm + ::std::fmt::Display,
    Rhs: G::BooleanFactor + ::std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} AND {}", self.lhs, self.rhs)
    }
}

impl<'q, DB, Lhs, Rhs> ToQuery<'q, DB> for And<Lhs, Rhs>
where
    DB: Database,
    Lhs: G::BooleanTerm + ToQuery<'q, DB>,
    Rhs: G::BooleanFactor + ToQuery<'q, DB>,
{
    fn write(&'q self, ctx: &mut crate::ToQueryContext<'q, DB>) -> ::std::fmt::Result {
        self.lhs.write(ctx)?;
        write!(ctx, " AND ")?;
        self.rhs.write(ctx)
    }
}

#[inline]
pub fn and<Lhs, Rhs>(lhs: Lhs, rhs: Rhs) -> And<Lhs, Rhs>
where
    Lhs: G::BooleanTerm,
    Rhs: G::BooleanFactor,
{
    And { lhs, rhs }
}

#[cfg(test)]
mod tests {
    use crate::{and, eq, id, neq};

    #[test]
    pub fn test_and() {
        let cond1 = eq(id("a"), id("b"));
        let cond2 = neq(id("c"), id("d"));

        let term = and(cond1, cond2);
        let sql = term.to_string();

        assert_eq!(sql, "a = b AND c <> d");
    }
}
