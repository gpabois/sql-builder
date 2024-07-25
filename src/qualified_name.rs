use sql_builder_macros::QualifiedName;

use crate::{grammar as G, ToQuery};
#[derive(QualifiedName)]
pub struct QualifiedName<SchemaName, Name>
where
    SchemaName: G::SchemaName,
    Name: G::Identifier,
{
    schema_name: SchemaName,
    name: Name,
}

impl<SchemaName, Name> ToQuery for QualifiedName<SchemaName, Name>
where
    SchemaName: G::SchemaName, Name: G::Identifier
{
    fn write<W: std::io::Write>(
        &self,
        stream: &mut W,
        ctx: &mut crate::ToQueryContext,
    ) -> Result<(), std::io::Error> {
        self.schema_name.write(stream, ctx)?;
        write!(stream, ".")?;
        self.name.write(stream, ctx)
    }
}
