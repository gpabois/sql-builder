use sql_builder_macros::BooleanTerm;

use crate::{grammar as G, Database, ToQuery};

#[derive(BooleanTerm)]
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

impl<DB, Lhs, Rhs> ToQuery<DB> for And<Lhs, Rhs>
where
    DB: Database,
    Lhs: G::BooleanTerm + ToQuery<DB>,
    Rhs: G::BooleanFactor + ToQuery<DB>,
{
    fn write<W: std::io::Write>(
        &self,
        stream: &mut W,
        ctx: &mut crate::ToQueryContext<DB>,
    ) -> Result<(), std::io::Error> {
        self.lhs.write(stream, ctx)?;
        write!(stream, " AND ")?;
        self.rhs.write(stream, ctx)
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
