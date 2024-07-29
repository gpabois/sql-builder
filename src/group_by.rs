use sql_builder_macros::GroupByClause;

use crate::{Database, ToQuery};

#[derive(GroupByClause)]
pub struct GroupBy();

impl ::std::fmt::Display for GroupBy {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}

impl<DB> ToQuery<DB> for GroupBy
where
    DB: Database,
{
    fn write<W: std::io::prelude::Write>(
        &self,
        stream: &mut W,
        _ctx: &mut crate::ToQueryContext<DB>,
    ) -> Result<(), std::io::Error> {
        write!(stream, "GROUP BY ")?;
        todo!("implement ToQuery for GroupBy")
    }
}
