use sql_builder_macros::HavingClause;

use crate::{Database, ToQuery};

#[derive(HavingClause)]
pub struct Having;

impl ::std::fmt::Display for Having {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}

impl<'q, DB> ToQuery<'q, DB> for Having
where
    DB: Database,
{
    fn write(&'q self, _ctx: &mut crate::ToQueryContext<'q, DB>) -> std::fmt::Result {
        todo!("implement ToQuery for Having")
    }
}
