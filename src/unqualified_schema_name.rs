use crate::{grammar::QualifiedIdentifier, Database, ToQuery};
use sql_builder_macros::UnqualifiedSchemaName;

#[derive(UnqualifiedSchemaName)]
pub struct UnqualifiedSchemaName<Id: QualifiedIdentifier>(Id);

impl<Id> ::std::fmt::Display for UnqualifiedSchemaName<Id>
where
    Id: QualifiedIdentifier + std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<DB, Id: QualifiedIdentifier> ToQuery<DB> for UnqualifiedSchemaName<Id>
where
    DB: Database,
    Id: QualifiedIdentifier + ToQuery<DB>,
{
    fn write<W: std::io::prelude::Write>(
        &self,
        stream: &mut W,
        ctx: &mut crate::ToQueryContext<DB>,
    ) -> Result<(), std::io::Error> {
        self.0.write(stream, ctx)
    }
}
