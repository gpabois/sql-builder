use sql_builder_macros::SchemaName;

use crate::{
    grammar::{Identifier, UnqualifiedSchemaName},
    Database, ToQuery,
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

impl<CatName, SchemName> ::std::fmt::Display for SchemaName<CatName, SchemName>
where
    CatName: Identifier + std::fmt::Display,
    SchemName: UnqualifiedSchemaName + std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}", self.catalog_name, self.unqualified_schema_name)
    }
}

impl<DB, CatName, SchemName> ToQuery<DB> for SchemaName<CatName, SchemName>
where
    DB: Database,
    CatName: Identifier + ToQuery<DB>,
    SchemName: UnqualifiedSchemaName + ToQuery<DB>,
{
    fn write<W: std::io::prelude::Write>(
        &self,
        stream: &mut W,
        ctx: &mut crate::ToQueryContext<DB>,
    ) -> Result<(), std::io::Error> {
        self.catalog_name.write(stream, ctx)?;
        write!(stream, ".")?;
        self.unqualified_schema_name.write(stream, ctx)
    }
}
