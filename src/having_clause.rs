use sql_builder_macros::HavingClause;

use crate::{Database, ToQuery};

#[derive(HavingClause)]
pub struct Having;

impl ::std::fmt::Display for Having {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}

impl<DB> ToQuery<DB> for Having
where
    DB: Database,
{
    fn write<W: std::io::Write>(
        &self,
        _stream: &mut W,
        _ctx: &mut crate::ToQueryContext<DB>,
    ) -> Result<(), std::io::Error> {
        todo!("implement ToQuery for Having")
    }
}
