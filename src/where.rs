use crate::{
    either::Either,
    grammar::{self, SearchCondition, WhereClause},
    ToQuery,
};

/// WHERE <search_condition>
pub struct Where<SearchCond: grammar::SearchCondition> {
    search_cond: SearchCond,
}

impl<SearchCond> Where<SearchCond>
where
    SearchCond: SearchCondition,
{
    pub fn new(search_cond: SearchCond) -> Self {
        Self { search_cond }
    }
}

impl<SearchCond> WhereClause for Where<SearchCond>
where
    SearchCond: SearchCondition,
{
    const IS_IMPL: bool = true;
}

impl<SearchCond> ToQuery for Where<SearchCond>
where
    SearchCond: grammar::SearchCondition,
{
    fn write<W: std::io::Write>(
        &self,
        stream: &mut W,
        ctx: &mut crate::ToQueryContext,
    ) -> Result<(), std::io::Error> {
        write!(stream, "WHERE ")?;
        self.search_cond.write(stream, ctx)
    }
}

impl<Lhs, Rhs> WhereClause for Either<Lhs, Rhs>
where
    Lhs: WhereClause,
    Rhs: WhereClause,
{
    const IS_IMPL: bool = true;
}
