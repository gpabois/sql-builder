use sql_builder_macros::FromConstructor;

use crate::{grammar as G, Database, ToQuery};

#[derive(FromConstructor)]
pub struct FromConstructor<Columns, Override, Value>
where
    Columns: G::InsertColumnList,
    Override: G::OverrideClause,
    Value: G::ContextuallyTypedTableValueConstructor,
{
    columns: Columns,
    override_clause: Override,
    value: Value,
}

impl<Columns, Override, Value> FromConstructor<Columns, Override, Value>
where
    Columns: G::InsertColumnList,
    Override: G::OverrideClause,
    Value: G::ContextuallyTypedTableValueConstructor,
{
    pub fn new(columns: Columns, override_clause: Override, value: Value) -> Self {
        Self {
            columns,
            override_clause,
            value,
        }
    }
}

impl<Columns, Override, Value> ::std::fmt::Display for FromConstructor<Columns, Override, Value>
where
    Columns: G::InsertColumnList + std::fmt::Display,
    Override: G::OverrideClause + std::fmt::Display,
    Value: G::ContextuallyTypedTableValueConstructor + std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if Columns::IS_IMPL {
            write!(f, "({}) ", self.columns)?;
        }

        if Override::IS_IMPL {
            write!(f, "{}", self.override_clause)?;
        }

        write!(f, "{}", self.value)
    }
}

impl<DB, Columns, Override, Value> ToQuery<DB> for FromConstructor<Columns, Override, Value>
where
    DB: Database,
    Columns: G::InsertColumnList + ToQuery<DB>,
    Override: G::OverrideClause + ToQuery<DB>,
    Value: G::ContextuallyTypedTableValueConstructor + ToQuery<DB>,
{
    fn write<W: std::io::Write>(
        &self,
        stream: &mut W,
        ctx: &mut crate::ToQueryContext<DB>,
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
