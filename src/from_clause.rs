use sql_builder_macros::FromClause;

use crate::{either::Either, table_expression::TableExpr, Blank, ToQuery};

use crate::grammar as G;
use crate::helpers as H;

#[derive(FromClause)]
pub struct From<Refs>
where
    Refs: G::TableReferenceList,
{
    table_refs: Refs,
}

impl<Refs> From<Refs>
where
    Refs: G::TableReferenceList,
{
    pub fn new(table_refs: Refs) -> Self {
        Self { table_refs }
    }
}

impl<Refs> H::FromClause for From<Refs>
where
    Refs: G::TableReferenceList,
{
    fn add_table_reference(self, table_ref: impl G::TableReference) -> impl G::FromClause {
        let table_refs = self.table_refs.add_table_reference(table_ref);

        From { table_refs }
    }
}

impl<TableRefs> H::TableExpression for From<TableRefs>
where
    TableRefs: G::TableReferenceList,
{
    type FromClause = Self;
    type WhereClause = Blank;

    fn transform_from<NewFromClause>(
        self,
        transform: impl FnOnce(Self::FromClause) -> NewFromClause,
    ) -> impl G::TableExpression
    where
        NewFromClause: G::FromClause,
    {
        TableExpr {
            from_clause: transform(self),
            where_clause: Blank,
            group_by: Blank,
            having: Blank,
        }
    }

    fn transform_where<NewWhereClause: G::WhereClause>(
        self,
        transform: impl FnOnce(Self::WhereClause) -> NewWhereClause,
    ) -> impl G::TableExpression {
        TableExpr {
            from_clause: self,
            where_clause: transform(Blank),
            group_by: Blank,
            having: Blank,
        }
    }
}

impl<TabRefs> ToQuery for From<TabRefs>
where
    TabRefs: G::TableReferenceList,
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

impl H::FromClause for Blank {
    fn add_table_reference(self, table_ref: impl G::TableReference) -> impl G::FromClause {
        let table_refs = table_ref.to_list();
        From::new(table_refs)
    }
}

impl<Lhs, Rhs> H::FromClause for Either<Lhs, Rhs>
where
    Lhs: G::FromClause,
    Rhs: G::FromClause,
{
    fn add_table_reference(self, table_refs: impl G::TableReference) -> impl G::FromClause {
        self.apply_with_args(
            table_refs,
            |lhs, table_refs| lhs.add_table_reference(table_refs),
            |rhs, table_refs| rhs.add_table_reference(table_refs),
        )
    }
}

#[cfg(test)]
mod test {
    use super::From;
    use crate::{identifier::id, ToQuery};

    #[test]
    fn test_from_identifier() {
        let clause = From::new(id("my_table"));
        let sql = clause.to_raw_query().unwrap();
        assert_eq!(sql, "FROM my_table");
    }
}
