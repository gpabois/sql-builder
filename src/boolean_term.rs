use sql_builder_macros::BooleanTerm;

use crate::{grammar::{self, BooleanFactor, BooleanTerm}, ToQuery};

#[derive(BooleanTerm)]
pub struct And<BoolTerm, BoolFactor>
where
BoolTerm: grammar::BooleanTerm,
BoolFactor: grammar::BooleanFactor
{
    lhs: BoolTerm,
    rhs: BoolFactor
}


impl<BoolTerm, BoolFactor> ToQuery for And<BoolTerm, BoolFactor>
where
    BoolTerm: grammar::BooleanTerm,
    BoolFactor: grammar::BooleanFactor,
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
pub fn and(lhs: impl BooleanTerm, rhs: impl BooleanFactor) -> impl BooleanTerm {
    And{lhs, rhs}
}

#[cfg(test)]
mod tests {
    use crate::{and, eq, id, lit, neq, ToQuery as _};

    #[test]
    pub fn test_and() {
        let cond1 = eq(id("a"), lit("b"));
        let cond2 = neq(id("c"), id("d"));

        let term = and(cond1, cond2);
        let sql = term.to_string().unwrap();

        assert_eq!(sql, "a = b AND c <> d");
    }
}