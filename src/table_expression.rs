use crate::{
    either::Either,
    grammar::{self, FromClause, GroupByClause, HavingClause, TableExpression, WhereClause},
    ToQuery,
};

/// A table expression
pub struct TableExpr<
    From: grammar::FromClause,
    Where: grammar::WhereClause,
    GroupBy: grammar::GroupByClause,
    Having: grammar::HavingClause,
> {
    pub(crate) from_clause: From,
    pub(crate) where_clause: Where,
    pub(crate) group_by: GroupBy,
    pub(crate) having: Having,
}

impl<From, Where, GroupBy, Having> TableExpression for TableExpr<From, Where, GroupBy, Having>
where
    From: FromClause,
    Where: WhereClause,
    GroupBy: GroupByClause,
    Having: HavingClause,
{
    type FromClause = From;
    type WhereClause = Where;

    fn transform_from<NewFromClause: FromClause>(
        self,
        transform: impl FnOnce(Self::FromClause) -> NewFromClause,
    ) -> impl TableExpression {
        TableExpr {
            from_clause: transform(self.from_clause),
            where_clause: self.where_clause,
            group_by: self.group_by,
            having: self.having,
        }
    }

    fn transform_where<NewWhereClause: WhereClause>(
        self,
        transform: impl FnOnce(Self::WhereClause) -> NewWhereClause,
    ) -> impl TableExpression {
        TableExpr {
            from_clause: self.from_clause,
            where_clause: transform(self.where_clause),
            group_by: self.group_by,
            having: self.having,
        }
    }
}

impl<From, Where, GroupBy, Having> ToQuery for TableExpr<From, Where, GroupBy, Having>
where
    From: FromClause,
    Where: WhereClause,
    GroupBy: GroupByClause,
    Having: HavingClause,
{
    fn write<W: std::io::Write>(
        &self,
        stream: &mut W,
        ctx: &mut crate::ToQueryContext,
    ) -> Result<(), std::io::Error> {
        self.from_clause.write(stream, ctx)?;

        if Where::IS_IMPL {
            write!(stream, " ")?;
            self.where_clause.write(stream, ctx)?;
        }

        if GroupBy::IS_IMPL {
            write!(stream, " ")?;
            self.group_by.write(stream, ctx)?;
        }

        if Having::IS_IMPL {
            write!(stream, " ")?;
            self.having.write(stream, ctx)?;
        }

        Ok(())
    }
}

impl<Lhs, Rhs> TableExpression for Either<Lhs, Rhs>
where
    Lhs: TableExpression,
    Rhs: TableExpression,
{
    type FromClause = Either<Lhs::FromClause, Rhs::FromClause>;
    type WhereClause = Either<Lhs::WhereClause, Rhs::WhereClause>;

    fn transform_from<NewFromClause: FromClause>(
        self,
        transform: impl FnOnce(Self::FromClause) -> NewFromClause,
    ) -> impl TableExpression {
        match self {
            Either::Left(lhs) => {
                Either::Left(lhs.transform_from(|from_clause| transform(Either::Left(from_clause))))
            }
            Either::Right(rhs) => Either::Right(
                rhs.transform_from(|from_clause| transform(Either::Right(from_clause))),
            ),
        }
    }

    fn transform_where<NewWhereClause: WhereClause>(
        self,
        transform: impl FnOnce(Self::WhereClause) -> NewWhereClause,
    ) -> impl TableExpression {
        match self {
            Either::Left(lhs) => Either::Left(
                lhs.transform_where(|where_clause| transform(Either::Left(where_clause))),
            ),
            Either::Right(rhs) => Either::Right(
                rhs.transform_where(|where_clause| transform(Either::Right(where_clause))),
            ),
        }
    }
}
