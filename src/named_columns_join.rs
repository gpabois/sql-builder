use sql_builder_macros::NamedColumnsJoin;

use crate::grammar as G;

#[derive(NamedColumnsJoin)]
pub struct NamedColumnsJoin<Cols>(Cols)
where
    Cols: G::JoinColumnList;

impl<Cols> NamedColumnsJoin<Cols>
where
    Cols: G::JoinColumnList,
{
    pub fn new(columns: Cols) -> Self {
        Self(columns)
    }
}
