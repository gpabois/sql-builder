use crate::{grammar, ToQuery};
use sql_builder_macros::DerivedColumn;

#[derive(DerivedColumn)]
pub struct DerivedColumn<ValueExpr, ColName>
where
    ValueExpr: grammar::ValueExpression,
    ColName: grammar::ColumnName,
{
    pub(crate) value_expression: ValueExpr,
    pub(crate) alias: ColName,
}

impl<ValueExpr, ColName> ToQuery for DerivedColumn<ValueExpr, ColName>
where
    ValueExpr: grammar::ValueExpression,
    ColName: grammar::ColumnName,
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
