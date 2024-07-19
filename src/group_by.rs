use crate::{grammar::GroupByClause, ToQuery};

pub struct GroupBy();

impl GroupByClause for GroupBy {
    const IS_IMPL: bool = true;
}

impl ToQuery for GroupBy {
    fn write<W: std::io::prelude::Write>(
        &self,
        stream: &mut W,
        ctx: &mut crate::ToQueryContext,
    ) -> Result<(), std::io::Error> {
        todo!("implement ToQuery for GroupBy")
    }
}
