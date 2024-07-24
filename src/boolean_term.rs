use sql_builder_macros::BooleanTerm;

use crate::ToQuery;

use crate::grammar as G;
use crate::helpers as H;

#[derive(BooleanTerm)]
pub struct And<Lhs, Rhs>
where
    Lhs: G::BooleanTerm,
    Rhs: G::BooleanFactor,
{
    lhs: Lhs,
    rhs: Rhs,
}

impl<Lhs, Rhs> H::ValueExpression for And<Lhs, Rhs>
where
    Lhs: G::BooleanTerm,
    Rhs: G::BooleanFactor,
{
}

impl<Lhs, Rhs> H::SelectSublist for And<Lhs, Rhs>
where
    Lhs: G::BooleanTerm,
    Rhs: G::BooleanFactor,
{
}

impl<Lhs, Rhs> H::SearchCondition for And<Lhs, Rhs>
where
    Lhs: G::BooleanTerm,
    Rhs: G::BooleanFactor,
{
}

impl<Lhs, Rhs> ToQuery for And<Lhs, Rhs>
where
    Lhs: G::BooleanTerm,
    Rhs: G::BooleanFactor,
{
    fn write<W: std::io::Write>(
        &self,
        stream: &mut W,
        ctx: &mut crate::ToQueryContext,
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
    use crate::{and, eq, id, neq, ToQuery as _};

    #[test]
    pub fn test_and() {
        let cond1 = eq(id("a"), id("b"));
        let cond2 = neq(id("c"), id("d"));

        let term = and(cond1, cond2);
        let sql = term.to_raw_query().unwrap();

        assert_eq!(sql, "a = b AND c <> d");
    }
}
