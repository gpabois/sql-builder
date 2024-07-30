use crate::{
    grammar::{Identifier, UnqualifiedSchemaName},
    Database, ToQuery,
};
use sql_builder_macros::SchemaName;
use std::fmt::Write;

#[derive(Clone, Copy, SchemaName)]
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

impl<'q, DB, CatName, SchemName> ToQuery<'q, DB> for SchemaName<CatName, SchemName>
where
    DB: Database,
    CatName: Identifier + ToQuery<'q, DB>,
    SchemName: UnqualifiedSchemaName + ToQuery<'q, DB>,
{
    fn write(&'q self, ctx: &mut crate::ToQueryContext<'q, DB>) -> std::fmt::Result {
        self.catalog_name.write(ctx)?;
        write!(ctx, ".")?;
        self.unqualified_schema_name.write(ctx)
    }
}
