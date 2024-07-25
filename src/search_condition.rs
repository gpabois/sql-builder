use crate::grammar as G;

use crate::ToQuery;
use sql_builder_macros::SearchCondition;

#[derive(SearchCondition)]
pub struct Or<Lhs, Rhs>
where
    Lhs: G::SearchCondition,
    Rhs: G::BooleanTerm,
{
    lhs: Lhs,
    rhs: Rhs,
}

impl<SearchCond, BoolTerm> ToQuery for Or<SearchCond, BoolTerm>
where
    SearchCond: G::SearchCondition,
    BoolTerm: G::BooleanTerm,
{
    fn write<W: std::io::Write>(
        &self,
        stream: &mut W,
        ctx: &mut crate::ToQueryContext,
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
