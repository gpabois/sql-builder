use crate::grammar as G;
use crate::Database;
use crate::ToQuery;
use sql_builder_macros::BooleanPrimary;
use std::fmt::Write;

#[derive(BooleanPrimary)]
pub struct NestedSearchCondition<Cond>(pub(crate) Cond)
where
    Cond: G::SearchCondition;

impl<Cond> NestedSearchCondition<Cond>
where
    Cond: G::SearchCondition,
{
    pub fn new(cond: Cond) -> Self {
        Self(cond)
    }
}

impl<SearchCond> ::std::fmt::Display for NestedSearchCondition<SearchCond>
where
    SearchCond: G::SearchCondition + ::std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({})", self.0)
    }
}

impl<'q, DB, SearchCond> ToQuery<'q, DB> for NestedSearchCondition<SearchCond>
where
    DB: Database,
    SearchCond: G::SearchCondition + ToQuery<'q, DB>,
{
    fn write(&'q self, ctx: &mut crate::ToQueryContext<'q, DB>) -> ::std::fmt::Result {
        write!(ctx, "(")?;
        self.0.write(ctx)?;
        write!(ctx, ")")
    }
}
