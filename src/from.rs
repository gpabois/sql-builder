use crate::{table::TableExpr, traits, ToQuery};

impl traits::FromExpr for () {
    const IS_IMPL: bool = false;
}

pub struct FromExpr {
    table: TableExpr,
}

impl ToQuery for FromExpr {
    fn write<W: std::io::prelude::Write>(
        &self,
        stream: &mut W,
        ctx: &mut crate::ToQueryContext,
    ) -> Result<(), std::io::Error> {
        write!(stream, "FROM ")?;
        self.table.write(stream, ctx)
    }
}

impl traits::FromExpr for FromExpr {
    const IS_IMPL: bool = true;
}
