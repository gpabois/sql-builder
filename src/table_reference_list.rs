use sql_builder_macros::TableReferenceList;

use crate::ToQuery;
use crate::grammar as G;
use crate::helpers as H;

#[derive(TableReferenceList)]
pub struct TableReferenceLink<Head, Tail>
where
    Head: G::TableReferenceList,
    Tail: G::TableReference,
{
    pub(crate) head: Head,
    pub(crate) tail: Tail,
}

impl<Head, Tail> ToQuery for TableReferenceLink<Head, Tail>
where
    Head: G::TableReferenceList,
    Tail: G::TableReferenceList,
{
    fn write<W: std::io::Write>(
        &self,
        stream: &mut W,
        ctx: &mut crate::ToQueryContext,
    ) -> Result<(), std::io::Error> {
        self.head.write(stream, ctx)?;
        write!(stream, ", ")?;
        self.tail.write(stream, ctx)
    }
}

