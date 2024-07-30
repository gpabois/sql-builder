use crate::{grammar as G, Database, ToQuery};
use sql_builder_macros::FromConstructor;
use std::fmt::Write;

#[derive(Clone, Copy, FromConstructor)]
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

impl<'q, DB, Columns, Override, Value> ToQuery<'q, DB> for FromConstructor<Columns, Override, Value>
where
    DB: Database,
    Columns: G::InsertColumnList + ToQuery<'q, DB>,
    Override: G::OverrideClause + ToQuery<'q, DB>,
    Value: G::ContextuallyTypedTableValueConstructor + ToQuery<'q, DB>,
{
    fn write(&'q self, ctx: &mut crate::ToQueryContext<'q, DB>) -> ::std::fmt::Result {
        if Columns::IS_IMPL {
            write!(ctx, "(")?;
            self.columns.write(ctx)?;
            write!(ctx, ") ")?;
        }

        if Override::IS_IMPL {
            self.override_clause.write(ctx)?;
            write!(ctx, " ")?;
        }

        self.value.write(ctx)
    }
}
