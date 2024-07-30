use crate::grammar as G;
use crate::Database;
use crate::ToQuery;
use sql_builder_macros::WhereClause;
use std::fmt::Write;

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

impl<'q, DB, SearchCond> ToQuery<'q, DB> for Where<SearchCond>
where
    DB: Database,
    SearchCond: G::SearchCondition + ToQuery<'q, DB>,
{
    fn write(&'q self, ctx: &mut crate::ToQueryContext<'q, DB>) -> std::fmt::Result {
        write!(ctx, "WHERE ")?;
        self.search_cond.write(ctx)
    }
}
