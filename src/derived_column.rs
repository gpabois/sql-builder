use crate::grammar as G;
use crate::Database;
use crate::ToQuery;
use sql_builder_macros::DerivedColumn;
use std::fmt::Write;

#[derive(DerivedColumn)]
pub struct AliasedColumn<Value, Name>
where
    Value: G::ValueExpression,
    Name: G::ColumnName,
{
    value_expression: Value,
    alias: Name,
}

impl<Value, Name> AliasedColumn<Value, Name>
where
    Value: G::ValueExpression,
    Name: G::ColumnName,
{
    pub fn new(value_expression: Value, alias: Name) -> Self {
        Self {
            value_expression,
            alias,
        }
    }
}

impl<Value, Name> ::std::fmt::Display for AliasedColumn<Value, Name>
where
    Value: G::ValueExpression + std::fmt::Display,
    Name: G::ColumnName + std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} AS {}", self.value_expression, self.alias)
    }
}

impl<'q, DB, Value, Name> ToQuery<'q, DB> for AliasedColumn<Value, Name>
where
    DB: Database,
    Value: G::ValueExpression + ToQuery<'q, DB>,
    Name: G::ColumnName + ToQuery<'q, DB>,
{
    fn write(&'q self, ctx: &mut crate::ToQueryContext<'q, DB>) -> std::fmt::Result {
        self.value_expression.write(ctx)?;
        write!(ctx, " AS ")?;
        self.alias.write(ctx)
    }
}
