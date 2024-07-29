use sql_builder_macros::WhereClause;

use crate::grammar as G;
use crate::Database;
use crate::ToQuery;

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

impl<SearchCond> ::std::fmt::Display for Where<SearchCond>
where
    SearchCond: G::SearchCondition + std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "WHERE {}", self.search_cond)
    }
}

impl<DB, SearchCond> ToQuery<DB> for Where<SearchCond>
where
    DB: Database,
    SearchCond: G::SearchCondition + ToQuery<DB>,
{
    fn write<W: std::io::Write>(
        &self,
        stream: &mut W,
        ctx: &mut crate::ToQueryContext<DB>,
    ) -> Result<(), std::io::Error> {
        write!(stream, "WHERE ")?;
        self.search_cond.write(stream, ctx)
    }
}

