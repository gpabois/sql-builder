use sql_builder_macros::FromConstructor;

use crate::{grammar as G, ToQuery};

#[derive(FromConstructor)]
pub struct FromConstructor<Columns, Override, Value>
where Columns: G::InsertColumnList, Override: G::OverrideClause, Value: G::ContextuallyTypedTableValueConstructor
{
    columns: Columns,
    override_clause: Override,
    value: Value 
}

impl<Columns, Override, Value> FromConstructor<Columns, Override, Value>
where Columns: G::InsertColumnList, Override: G::OverrideClause, Value: G::ContextuallyTypedTableValueConstructor
{
    pub fn new(columns: Columns, override_clause: Override, value: Value) -> Self {
        Self { columns, override_clause, value }
    }
}

impl<Columns, Override, Value> ToQuery for FromConstructor<Columns, Override, Value>
where Columns: G::InsertColumnList, Override: G::OverrideClause, Value: G::ContextuallyTypedTableValueConstructor
{
    fn write<W: std::io::Write>(
        &self,
        stream: &mut W,
        ctx: &mut crate::ToQueryContext,
    ) -> Result<(), std::io::Error> {
        if Columns::IS_IMPL {
            write!(stream, "(")?;
            self.columns.write(stream, ctx)?;
            write!(stream, ") ")?;
        }

        if Override::IS_IMPL {
            self.override_clause.write(stream, ctx)?;
            write!(stream, " ")?;
        }

        self.value.write(stream, ctx)
    }
}
