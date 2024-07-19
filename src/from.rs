use crate::{grammar::{FromClause, SearchCondition, TableExpression, TableReferenceList}, table_expression::TableExpr, r#where::Where, ToQuery};

pub struct From<TabRefs> where TabRefs: TableReferenceList {
    pub(crate) table_refs: TabRefs,
}

impl<TabRefs> From<TabRefs> where TabRefs: TableReferenceList {
    pub fn new(table_refs: TabRefs) -> Self {
        Self {table_refs}
    }
}

impl<TabRefs> FromClause for From<TabRefs> 
where TabRefs: TableReferenceList {
    fn add_table_references(self, table_refs: impl TableReferenceList) -> impl FromClause {
        From {
            table_refs: self.table_refs.chain(table_refs)
        }
    }
}

impl<TableRefs> TableExpression for From<TableRefs> where TableRefs: TableReferenceList 
{
    fn r#where(self, search_condition: impl SearchCondition) -> impl TableExpression {
        TableExpr {
            from_clause: self,
            where_clause: Where::new(search_condition),
            group_by: (),
            having: (),
        }
    }
        
    fn and_from(self, table_refs: impl TableReferenceList) -> impl TableExpression 
    {
        TableExpr {
            from_clause: self.add_table_references(table_refs),
            where_clause: (),
            group_by: (),
            having: ()
        }
    }
}

impl<TabRefs> ToQuery for From<TabRefs>
where
    TabRefs: TableReferenceList,
{
    fn write<W: std::io::prelude::Write>(
        &self,
        stream: &mut W,
        ctx: &mut crate::ToQueryContext,
    ) -> Result<(), std::io::Error> {
        write!(stream, "FROM ")?;
        self.table_refs.write(stream, ctx)
    }
}

#[cfg(test)]
mod test {
    use super::From;
    use crate::{identifier::id, ToQuery};

    #[test]
    fn test_from_identifier() {
        let clause = From::new(id("my_table"));
        let sql = clause.to_string().unwrap();
        assert_eq!(sql, "FROM my_table");
    }
}

