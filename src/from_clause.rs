use crate::grammar as G;
use crate::helpers as H;
use crate::Database;
use crate::{blank::Blank, either::Either, table_expression::TableExpr, ToQuery};
use sql_builder_macros::FromClause;
use std::fmt::Write;

#[derive(Clone, Copy, FromClause)]
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
    type GroupByClause = Blank;
    type HavingClause = Blank;

    fn unwrap(
        self,
    ) -> TableExpr<Self::FromClause, Self::WhereClause, Self::GroupByClause, Self::HavingClause>
    {
        TableExpr {
            from_clause: self,
            where_clause: Blank,
            group_by: Blank,
            having: Blank,
        }
    }
}

impl<TabRefs> ::std::fmt::Display for From<TabRefs>
where
    TabRefs: G::TableReferenceList + std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FROM {}", self.table_refs)
    }
}

impl<'q, DB, TabRefs> ToQuery<'q, DB> for From<TabRefs>
where
    DB: Database,
    TabRefs: G::TableReferenceList + ToQuery<'q, DB>,
{
    fn write(&'q self, ctx: &mut crate::ToQueryContext<'q, DB>) -> std::fmt::Result {
        write!(ctx, "FROM ")?;
        self.table_refs.write(ctx)
    }
}

impl H::FromClause for Blank {
    fn add_table_reference(self, table_ref: impl G::TableReference) -> impl G::FromClause {
        From::new(table_ref)
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
    use crate::identifier::id;

    #[test]
    fn test_from_identifier() {
        let clause = From::new(id("my_table"));
        let sql = clause.to_string();
        assert_eq!(sql, "FROM my_table");
    }
}
