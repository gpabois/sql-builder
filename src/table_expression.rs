use crate::{blank::Blank, either::Either, Database, ToQuery};
use sql_builder_macros::TableExpression;
use std::fmt::Write;

use crate::grammar as G;
use crate::helpers as H;

#[derive(TableExpression)]
/// A table expression
pub struct TableExpr<
    From: G::FromClause,
    Where: G::WhereClause,
    GroupBy: G::GroupByClause,
    Having: G::HavingClause,
> {
    pub from_clause: From,
    pub where_clause: Where,
    pub group_by: GroupBy,
    pub having: Having,
}

impl<From, Where, GroupBy, Having> H::TableExpression for TableExpr<From, Where, GroupBy, Having>
where
    From: G::FromClause,
    Where: G::WhereClause,
    GroupBy: G::GroupByClause,
    Having: G::HavingClause,
{
    type FromClause = From;
    type WhereClause = Where;
    type GroupByClause = GroupBy;
    type HavingClause = Having;

    fn unwrap(
        self,
    ) -> TableExpr<Self::FromClause, Self::WhereClause, Self::GroupByClause, Self::HavingClause>
    {
        self
    }
}

impl<From, Where, GroupBy, Having> std::fmt::Display for TableExpr<From, Where, GroupBy, Having>
where
    From: G::FromClause + std::fmt::Display,
    Where: G::WhereClause + std::fmt::Display,
    GroupBy: G::GroupByClause + std::fmt::Display,
    Having: G::HavingClause + std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.from_clause)?;

        if Where::IS_IMPL {
            write!(f, " {}", self.where_clause)?;
        }

        if GroupBy::IS_IMPL {
            write!(f, " {}", self.group_by)?;
        }

        if Having::IS_IMPL {
            write!(f, " {}", self.having)?;
        }

        Ok(())
    }
}

impl<'q, DB, From, Where, GroupBy, Having> ToQuery<'q, DB>
    for TableExpr<From, Where, GroupBy, Having>
where
    DB: Database,
    From: G::FromClause + ToQuery<'q, DB>,
    Where: G::WhereClause + ToQuery<'q, DB>,
    GroupBy: G::GroupByClause + ToQuery<'q, DB>,
    Having: G::HavingClause + ToQuery<'q, DB>,
{
    fn write(&'q self, ctx: &mut crate::ToQueryContext<'q, DB>) -> std::fmt::Result {
        self.from_clause.write(ctx)?;

        if Where::IS_IMPL {
            write!(ctx, " ")?;
            self.where_clause.write(ctx)?;
        }

        if GroupBy::IS_IMPL {
            write!(ctx, " ")?;
            self.group_by.write(ctx)?;
        }

        if Having::IS_IMPL {
            write!(ctx, " ")?;
            self.having.write(ctx)?;
        }

        Ok(())
    }
}

impl H::TableExpression for Blank {
    type FromClause = Blank;
    type WhereClause = Blank;
    type HavingClause = Blank;
    type GroupByClause = Blank;

    fn unwrap(
        self,
    ) -> TableExpr<Self::FromClause, Self::WhereClause, Self::GroupByClause, Self::HavingClause>
    {
        TableExpr {
            from_clause: Blank,
            where_clause: Blank,
            group_by: Blank,
            having: Blank,
        }
    }
}

impl<Lhs, Rhs> H::TableExpression for Either<Lhs, Rhs>
where
    Lhs: H::TableExpression,
    Rhs: H::TableExpression,
{
    type FromClause = Either<Lhs::FromClause, Rhs::FromClause>;
    type WhereClause = Either<Lhs::WhereClause, Rhs::WhereClause>;
    type GroupByClause = Either<Lhs::GroupByClause, Rhs::GroupByClause>;
    type HavingClause = Either<Lhs::HavingClause, Rhs::HavingClause>;

    fn unwrap(
        self,
    ) -> TableExpr<Self::FromClause, Self::WhereClause, Self::GroupByClause, Self::HavingClause>
    {
        match self {
            Either::Left(left) => {
                let TableExpr {
                    from_clause,
                    where_clause,
                    group_by,
                    having,
                } = left.unwrap();
                TableExpr {
                    from_clause: Either::Left(from_clause),
                    where_clause: Either::Left(where_clause),
                    group_by: Either::Left(group_by),
                    having: Either::Left(having),
                }
            }
            Either::Right(right) => {
                let TableExpr {
                    from_clause,
                    where_clause,
                    group_by,
                    having,
                } = right.unwrap();
                TableExpr {
                    from_clause: Either::Right(from_clause),
                    where_clause: Either::Right(where_clause),
                    group_by: Either::Right(group_by),
                    having: Either::Right(having),
                }
            }
        }
    }
}
