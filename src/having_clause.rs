use sql_builder_macros::HavingClause;

use crate::ToQuery;

#[derive(HavingClause)]
pub struct Having;

impl ToQuery for Having {
    fn write<W: std::io::Write>(
        &self,
        _stream: &mut W,
        _ctx: &mut crate::ToQueryContext,
    ) -> Result<(), std::io::Error> {
        todo!("implement ToQuery for Having")
    }
}

