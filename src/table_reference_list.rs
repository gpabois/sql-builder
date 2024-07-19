use sql_builder_macros::TableReferenceList;

use crate::{grammar::TableReferenceList, ToQuery};

#[derive(TableReferenceList)]
pub struct TableRefList<Lhs, Rhs> 
where Lhs: TableReferenceList, 
      Rhs: TableReferenceList 
{
    pub(crate) lhs: Lhs,
    pub(crate) rhs: Rhs
}

impl<Lhs, Rhs> ToQuery for TableRefList<Lhs, Rhs> where Lhs: TableReferenceList, Rhs: TableReferenceList {
    fn write<W: std::io::Write>(
        &self,
        stream: &mut W,
        ctx: &mut crate::ToQueryContext,
    ) -> Result<(), std::io::Error> {
        self.lhs.write(stream, ctx)?;
        write!(stream, ", ")?;
        self.rhs.write(stream, ctx)
    }
}