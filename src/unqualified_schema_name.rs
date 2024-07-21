use crate::{grammar::QualifiedIdentifier, ToQuery};
use sql_builder_macros::UnqualifiedSchemaName;

#[derive(UnqualifiedSchemaName)]
pub struct UnqualifiedSchemaName<Id: QualifiedIdentifier>(Id);

impl<Id: QualifiedIdentifier> ToQuery for UnqualifiedSchemaName<Id> {
    fn write<W: std::io::prelude::Write>(
        &self,
        stream: &mut W,
        ctx: &mut crate::ToQueryContext,
    ) -> Result<(), std::io::Error> {
        self.0.write(stream, ctx)
    }
}
