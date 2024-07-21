use sql_builder_macros::{QualifiedName, UnqualifiedSchemaName};

use crate::{grammar, identifier::Identifier, ToQuery};
#[derive(QualifiedName)]
pub struct QualifiedName<SchemaName>
where
    SchemaName: grammar::SchemaName,
{
    schema_name: SchemaName,
    name: Identifier,
}

impl<SchemaName> ToQuery for QualifiedName<SchemaName>
where
    SchemaName: grammar::SchemaName,
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
