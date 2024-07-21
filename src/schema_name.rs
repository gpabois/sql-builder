use sql_builder_macros::SchemaName;

use crate::{
    grammar::{Identifier, UnqualifiedSchemaName},
    ToQuery,
};

#[derive(SchemaName)]
pub struct SchemaName<CatName, SchemName>
where
    CatName: Identifier,
    SchemName: UnqualifiedSchemaName,
{
    catalog_name: CatName,
    unqualified_schema_name: SchemName,
}

impl<CatName, SchemName> ToQuery for SchemaName<CatName, SchemName>
where
    CatName: Identifier,
    SchemName: UnqualifiedSchemaName,
{
    fn write<W: std::io::prelude::Write>(
        &self,
        stream: &mut W,
        ctx: &mut crate::ToQueryContext,
    ) -> Result<(), std::io::Error> {
        self.catalog_name.write(stream, ctx)?;
        write!(stream, ".")?;
        self.unqualified_schema_name.write(stream, ctx)
    }
}
