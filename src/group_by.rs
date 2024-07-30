use crate::{Database, ToQuery};
use sql_builder_macros::GroupByClause;
use std::fmt::Write;

#[derive(Clone, Copy, GroupByClause)]
pub struct GroupBy();

impl ::std::fmt::Display for GroupBy {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}

impl<'q, DB> ToQuery<'q, DB> for GroupBy
where
    DB: Database,
{
    fn write(&'q self, ctx: &mut crate::ToQueryContext<'q, DB>) -> std::fmt::Result {
        write!(ctx, "GROUP BY ")?;
        todo!("implement ToQuery for GroupBy")
    }
}
