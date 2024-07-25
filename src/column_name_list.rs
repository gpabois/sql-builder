use crate::grammar as G;
use crate::helpers as H;
use crate::ToQuery;

use sql_builder_macros::ColumnNameList;

#[derive(ColumnNameList)]
pub struct ColumnNameLink<Head, Tail> 
where Head: G::ColumnNameList, Tail: G::ColumnName
{
    head: Head,
    tail: Tail
}

impl<Head, Tail> H::ColumnNameList for ColumnNameLink<Head, Tail> 
where Head: G::ColumnNameList, Tail: G::ColumnName {
}

impl<Head, Tail> ToQuery for ColumnNameLink<Head, Tail> 
where Head: G::ColumnNameList, Tail: G::ColumnName {
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

impl<Head, Tail> ColumnNameLink<Head, Tail> 
where Head: G::ColumnNameList, Tail: G::ColumnName {
    pub fn new(head: Head, tail: Tail) -> Self {
        Self { head, tail }
    }
}

