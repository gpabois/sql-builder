use sql_builder_macros::GroupByClause;

use crate::ToQuery;


#[derive(GroupByClause)]
pub struct GroupBy();

impl ToQuery for GroupBy {
    fn write<W: std::io::prelude::Write>(
        &self,
        stream: &mut W,
        _ctx: &mut crate::ToQueryContext,
    ) -> Result<(), std::io::Error> {
        write!(stream, "GROUP BY ")?;
        todo!("implement ToQuery for GroupBy")
    }
}
