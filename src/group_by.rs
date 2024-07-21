use crate::{grammar::GroupByClause, Blank, ToQuery};

pub struct GroupBy();

impl GroupByClause for GroupBy {
    const IS_IMPL: bool = true;
}

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

impl GroupByClause for Blank {
    const IS_IMPL: bool = false;
}
