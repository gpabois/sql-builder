use crate::{grammar::{self, BooleanTerm, SearchCondition}, ToQuery};
use sql_builder_macros::SearchCondition;

#[derive(SearchCondition)]
pub struct Or<Lhs, Rhs>
where
    Lhs: grammar::SearchCondition,
    Rhs: grammar::BooleanTerm
{
    lhs: Lhs, 
    rhs: Rhs
}


impl<SearchCond, BoolTerm> ToQuery for Or<SearchCond, BoolTerm>
where
    SearchCond: grammar::SearchCondition,
    BoolTerm: grammar::BooleanTerm,
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
pub fn or(lhs: impl SearchCondition, rhs: impl BooleanTerm) -> impl SearchCondition {
    Or {lhs, rhs}
}