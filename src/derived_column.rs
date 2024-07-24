use crate::ToQuery;
use sql_builder_macros::DerivedColumn;

use crate::grammar as G;
use crate::helpers as H;

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

impl<Value, Name> H::SelectSublist for AliasedColumn<Value, Name>
where
    Value: G::ValueExpression,
    Name: G::ColumnName,
{
}

impl<Value, Name> ToQuery for AliasedColumn<Value, Name>
where
    Value: G::ValueExpression,
    Name: G::ColumnName,
{
    fn write<W: std::io::Write>(
        &self,
        stream: &mut W,
        ctx: &mut crate::ToQueryContext,
    ) -> Result<(), std::io::Error> {
        self.value_expression.write(stream, ctx)?;
        write!(stream, " AS ")?;
        self.alias.write(stream, ctx)
    }
}
