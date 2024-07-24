use sql_builder_macros::WhereClause;

use crate::ToQuery;
use crate::grammar as G;

#[derive(WhereClause)]
/// WHERE <search_condition>
pub struct Where<SearchCond: G::SearchCondition> {
    search_cond: SearchCond,
}

impl<SearchCond> Where<SearchCond>
where
    SearchCond: G::SearchCondition,
{
    pub fn new(search_cond: SearchCond) -> Self {
        Self { search_cond }
    }
}

impl<SearchCond> ToQuery for Where<SearchCond>
where
    SearchCond: G::SearchCondition,
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