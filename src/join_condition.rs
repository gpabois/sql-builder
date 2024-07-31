use crate::{grammar as G, ToQuery};
use sql_builder_macros::JoinCondition;
use sqlx::Database;
use std::fmt::Write;

#[derive(JoinCondition)]
pub struct JoinCondition<Cond>(Cond)
where
    Cond: G::SearchCondition;

impl<Cond> JoinCondition<Cond>
where
    Cond: G::SearchCondition,
{
    pub fn new(cond: Cond) -> Self {
        Self(cond)
    }
}

impl<Cond> ::std::fmt::Display for JoinCondition<Cond>
where
    Cond: G::SearchCondition + std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ON {}", self.0)
    }
}

impl<'q, DB, Cond> ToQuery<'q, DB> for JoinCondition<Cond>
where
    DB: Database,
    Cond: G::SearchCondition + ToQuery<'q, DB>,
{
    fn write(&'q self, ctx: &mut crate::ToQueryContext<'q, DB>) -> std::fmt::Result {
        write!(ctx, "ON ")?;
        self.0.write(ctx)
    }
}
