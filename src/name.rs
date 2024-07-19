use sql_builder_macros::{QualifiedName, SchemaName};

use crate::{identifier::Identifier, grammar, ToQuery};

#[derive(SchemaName)]
pub struct SchemaName {
    catalog_name: Identifier,
    unqualified_schema_name: UnqualifiedSchemaName,
}

pub struct UnqualifiedSchemaName(Identifier);
impl grammar::SchemaName for UnqualifiedSchemaName {}

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
        todo!("implement ToQuery for QualifiedName")
    }
}
