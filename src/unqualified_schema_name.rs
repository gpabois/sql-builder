use crate::{grammar::QualifiedIdentifier, Database, ToQuery};
use sql_builder_macros::UnqualifiedSchemaName;

#[derive(Clone, Copy, UnqualifiedSchemaName)]
pub struct UnqualifiedSchemaName<Id: QualifiedIdentifier>(Id);

impl<Id> ::std::fmt::Display for UnqualifiedSchemaName<Id>
where
    Id: QualifiedIdentifier + std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<'q, DB, Id: QualifiedIdentifier> ToQuery<'q, DB> for UnqualifiedSchemaName<Id>
where
    DB: Database,
    Id: QualifiedIdentifier + ToQuery<'q, DB>,
{
    fn write(&'q self, ctx: &mut crate::ToQueryContext<'q, DB>) -> std::fmt::Result {
        self.0.write(ctx)
    }
}
