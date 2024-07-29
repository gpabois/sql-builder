use crate::ToQuery;
use sql_builder_macros::DerivedColumn;

use crate::grammar as G;
use crate::Database;

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

impl<DB, Value, Name> ToQuery<DB> for AliasedColumn<Value, Name>
where
    DB: Database,
    Value: G::ValueExpression + ToQuery<DB>,
    Name: G::ColumnName + ToQuery<DB>,
{
    fn write<W: std::io::Write>(
        &self,
        stream: &mut W,
        ctx: &mut crate::ToQueryContext<DB>,
    ) -> Result<(), std::io::Error> {
        self.value_expression.write(stream, ctx)?;
        write!(stream, " AS ")?;
        self.alias.write(stream, ctx)
    }
}
