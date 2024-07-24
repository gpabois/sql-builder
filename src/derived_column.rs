use crate::{grammar, ToQuery};
use sql_builder_macros::DerivedColumn;

use crate::helpers as H;
use crate::grammar as G;

#[derive(DerivedColumn)]
pub struct AliasedColumn<Value, Name>
where
    Value: G::ValueExpression,
    Name: G::ColumnName,
{
    pub(crate) value_expression: Value,
    pub(crate) alias: Name,
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
