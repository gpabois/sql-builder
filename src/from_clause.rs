use sql_builder_macros::FromClause;

use crate::{either::Either, table_expression::TableExpr, Blank, ToQuery};

use crate::grammar as G;
use crate::helpers as H;

#[derive(FromClause)]
pub struct From<TabRefs>
where
    TabRefs: G::TableReferenceList,
{
    pub(crate) table_refs: TabRefs,
}

impl<TabRefs> From<TabRefs>
where
    TabRefs: G::TableReferenceList,
{
    pub fn new(table_refs: TabRefs) -> Self {
        Self { table_refs }
    }
}

impl<TabRefs> H::FromClause for From<TabRefs>
where
    TabRefs: G::TableReferenceList,
{
    fn add_table_references<TabRef>(self, table_refs: TabRef) -> impl G::FromClause
    where
        TabRef: G::TableReference,
    {
        From {
            table_refs: self.table_refs.add_table_reference(table_refs),
        }
    }
}

impl<TableRefs> H::TableExpression for From<TableRefs>
where
    TableRefs: G::TableReferenceList,
{
    type FromClause = Self;
    type WhereClause = Blank;

    fn transform_from<NewFromClause: G::FromClause>(
        self,
        transform: impl FnOnce(Self::FromClause) -> NewFromClause,
    ) -> impl G::TableExpression {
        transform(self)
    }

    fn transform_where<NewWhereClause: G::WhereClause>(
        self,
        transform: impl FnOnce(Self::WhereClause) -> NewWhereClause,
    ) -> impl G::TableExpression {
        TableExpr {
            from_clause: self,
            where_clause: transform(Blank()),
            group_by: Blank(),
            having: Blank(),
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

impl<Lhs, Rhs> G::FromClause for Either<Lhs, Rhs>
where
    Lhs: G::FromClause,
    Rhs: G::FromClause,
{
}

impl<Lhs, Rhs> H::FromClause for Either<Lhs, Rhs>
where
    Lhs: G::FromClause,
    Rhs: G::FromClause,
{
    fn add_table_references(self, tab_refs: impl G::TableReferenceList) -> impl G::FromClause {
        match self {
            Either::Left(left) => Either::Left(left.add_table_references(tab_refs)),
            Either::Right(right) => Either::Right(right.add_table_references(tab_refs)),
        }
    }
}

impl H::FromClause for Blank {
    fn add_table_references(self, table_refs: impl G::TableReferenceList) -> impl G::FromClause {
        From::new(table_refs)
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
