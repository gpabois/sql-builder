use crate::{
    either::Either,
    grammar::{FromClause, SearchCondition, TableExpression, TableReferenceList},
    r#where::Where,
    table_expression::TableExpr,
    ToQuery,
};

pub struct From<TabRefs>
where
    TabRefs: TableReferenceList,
{
    pub(crate) table_refs: TabRefs,
}

impl<TabRefs> From<TabRefs>
where
    TabRefs: TableReferenceList,
{
    pub fn new(table_refs: TabRefs) -> Self {
        Self { table_refs }
    }
}

impl<TabRefs> FromClause for From<TabRefs>
where
    TabRefs: TableReferenceList,
{
    fn add_table_references(self, table_refs: impl TableReferenceList) -> impl FromClause {
        From {
            table_refs: self.table_refs.chain(table_refs),
        }
    }
}

impl<TableRefs> TableExpression for From<TableRefs>
where
    TableRefs: TableReferenceList,
{
    type FromClause = Self;
    type WhereClause = ();

    fn transform_from<NewFromClause: FromClause>(
        self,
        transform: impl FnOnce(Self::FromClause) -> NewFromClause,
    ) -> impl TableExpression {
        transform(self)
    }

    fn transform_where<NewWhereClause: crate::grammar::WhereClause>(
        self,
        transform: impl FnOnce(Self::WhereClause) -> NewWhereClause,
    ) -> impl TableExpression {
        TableExpr {
            from_clause: self,
            where_clause: transform(()),
            group_by: (),
            having: (),
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

impl<Lhs: FromClause, Rhs: FromClause> FromClause for Either<Lhs, Rhs> {
    fn add_table_references(self, tab_refs: impl TableReferenceList) -> impl FromClause {
        match self {
            Either::Left(left) => Either::Left(left.add_table_references(tab_refs)),
            Either::Right(right) => Either::Right(right.add_table_references(tab_refs)),
        }
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
