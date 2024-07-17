use crate::{identifier::Identifier, traits, ToQuery};

/// <schema name> ::= [ <catalog name> <period> ] <unqualified schema name>
pub struct SchemaName {
    catalog_name: Identifier,
    unqualified_schema_name: UnqualifiedSchemaName
}
impl traits::SchemaName for SchemaName {}
pub struct UnqualifiedSchemaName(Identifier);
impl traits::SchemaName for UnqualifiedSchemaName {}

/// <qualified name> ::= [<schema name> <period>] <identifier>
pub struct QualifiedName<SchemaName> where SchemaName: traits::SchemaName {
    schema_name: SchemaName,
    name: Identifier
}

impl<SchemaName> traits::QualifiedName for QualifiedName<SchemaName> where SchemaName: traits:: SchemaName {}
impl<SchemaName> traits::TableReference for QualifiedName<SchemaName> where SchemaName: traits::SchemaName {}

impl<SchemaName> ToQuery for QualifiedName<SchemaName> where SchemaName: traits:: SchemaName  {
    fn write<W: std::io::Write>(
        &self,
        stream: &mut W,
        ctx: &mut crate::ToQueryContext,
    ) -> Result<(), std::io::Error> {
        todo!("implement ToQuery for QualifiedName")
    }
}


