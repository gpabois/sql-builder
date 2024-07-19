use crate::{grammar::{self, FromClause, GroupByClause, HavingClause, SearchCondition, TableExpression, TableReferenceList, WhereClause}, r#where::Where, ToQuery};

pub struct TableExpr<
    From: grammar::FromClause, 
    Where: grammar::WhereClause, 
    GroupBy: grammar::GroupByClause,
    Having: grammar::HavingClause,
> {
    pub(crate) from_clause: From,
    pub(crate) where_clause: Where,
    pub(crate) group_by: GroupBy,
    pub(crate) having: Having
}

impl<From, Where, GroupBy, Having> TableExpression for TableExpr<From, Where, GroupBy, Having>
where   From: FromClause, 
        Where: WhereClause, 
        GroupBy: GroupByClause,
        Having: HavingClause,
{
    fn r#where(self, search_condition: impl SearchCondition) -> impl TableExpression {
        TableExpr {
            from_clause: self.from_clause,
            where_clause: crate::Where::new(search_condition),
            group_by: self.group_by,
            having: self.having
        }
    }

    
    fn and_from(self, table_refs: impl TableReferenceList) -> impl grammar::TableExpression {
        TableExpr {
            from_clause: self.from_clause.add_table_references(table_refs),
            where_clause: self.where_clause,
            group_by: self.group_by,
            having: self.having,
        }
    } 

}

impl<From, Where, GroupBy, Having> ToQuery for TableExpr<From, Where, GroupBy, Having>
where   From: FromClause, 
        Where: WhereClause, 
        GroupBy: GroupByClause,
        Having: HavingClause,
{
    fn write<W: std::io::Write>(
        &self,
        stream: &mut W,
        ctx: &mut crate::ToQueryContext,
    ) -> Result<(), std::io::Error> {
        todo!("ToQuery not implemented for TableExpr")
    }
}