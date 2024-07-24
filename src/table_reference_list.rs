use sql_builder_macros::TableReferenceList;

use crate::ToQuery;
use crate::grammar as G;
use crate::helpers as H;

#[derive(TableReferenceList)]
pub struct TableReferenceListKernel<Ref>(Ref) where Ref: G::TableReference;

impl<Ref> TableReferenceListKernel<Ref> 
    where Ref: G::TableReference 
{
    pub fn new(table_ref: Ref) -> Self {
        Self(table_ref)
    }   
}

impl<Ref> H::TableReferenceList for TableReferenceListKernel<Ref> 
where Ref: G::TableReference
{
    fn add_table_reference(
        self,
        table_ref: impl G::TableReference,
    ) -> impl G::TableReferenceList {
        TableReferenceLink::new(self, table_ref)
    }
}

impl<Ref> ToQuery for TableReferenceListKernel<Ref> 
where Ref: G::TableReference
{
    fn write<W: std::io::Write>(
        &self,
        stream: &mut W,
        ctx: &mut crate::ToQueryContext,
    ) -> Result<(), std::io::Error> {
        self.0.write(stream, ctx)
    }
}

#[derive(TableReferenceList)]
pub struct TableReferenceLink<Head, Tail>
where
    Head: G::TableReferenceList,
    Tail: G::TableReference,
{
    pub(crate) head: Head,
    pub(crate) tail: Tail,
}

impl<Head, Tail> TableReferenceLink<Head, Tail>
where
    Head: G::TableReferenceList,
    Tail: G::TableReference 
{
    pub fn new(head: Head, tail: Tail) -> Self {
        Self{head, tail}
    }
}

impl<Head, Tail> H::TableReferenceList for TableReferenceLink<Head, Tail>
where
    Head: G::TableReferenceList,
    Tail: G::TableReference {
        fn add_table_reference(
        self,
        table_ref: impl G::TableReference,
    ) -> impl G::TableReferenceList
    where
        Self: G::TableReferenceList 
    {
        TableReferenceLink {
            head: self,
            tail: table_ref
        }
    }
}

impl<Head, Tail> ToQuery for TableReferenceLink<Head, Tail>
where
    Head: G::TableReferenceList,
    Tail: G::TableReference,
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

