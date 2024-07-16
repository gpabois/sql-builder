use crate::{
    alias::Alias,
    traits::{self, FromExpr, SelectExpr, WhereExpr},
    ToQuery,
};

pub enum SelectKind {
    Default,
    All,
    Distinct,
}

/// Select command
pub struct SelectStatement<
    Select: traits::SelectExpr,
    From: traits::FromExpr,
    Where: traits::WhereExpr,
    GroupBy: traits::GroupByExpr,
    Having: traits::HavingExpr,
    OrderBy: traits::OrderByExpr,
    Limit: traits::LimitExpr,
> {
    pub kind: SelectKind,
    pub select: Select,
    pub from: From,
    pub r#where: Where,
    pub group_by: GroupBy,
    pub having: Having,
    pub order_by: OrderBy,
    pub limit: Limit,
}

pub type BlankSelectStatement = SelectStatement<(), (), (), (), (), (), ()>;
pub type InitSelectStatement<S> = SelectStatement<S, (), (), (), (), (), ()>;

impl BlankSelectStatement {
    pub fn new<E: SelectExpr>(select: E) -> InitSelectStatement<E> {
        InitSelectStatement {
            kind: SelectKind::Default,
            select,
            from: (),
            r#where: (),
            group_by: (),
            having: (),
            order_by: (),
            limit: (),
        }
    }
}

impl<Select, From, Where, GroupBy, Having, OrderBy, Limit>
    SelectStatement<Select, From, Where, GroupBy, Having, OrderBy, Limit>
where
    Select: traits::SelectExpr,
    From: traits::FromExpr,
    Where: traits::WhereExpr,
    GroupBy: traits::GroupByExpr,
    Having: traits::HavingExpr,
    OrderBy: traits::OrderByExpr,
    Limit: traits::LimitExpr,
{
    pub fn from<E: FromExpr, F: Into<E>>(
        self,
        from: F,
    ) -> SelectStatement<Select, E, Where, GroupBy, Having, OrderBy, Limit> {
        SelectStatement {
            kind: self.kind,
            select: self.select,
            from: from.into(),
            r#where: self.r#where,
            group_by: self.group_by,
            having: self.having,
            order_by: self.order_by,
            limit: self.limit,
        }
    }

    pub fn r#where<E: WhereExpr, F: Into<E>>(
        self,
        r#where: F,
    ) -> SelectStatement<Select, From, E, GroupBy, Having, OrderBy, Limit> {
        SelectStatement {
            kind: self.kind,
            select: self.select,
            from: self.from,
            r#where: r#where.into(),
            group_by: self.group_by,
            having: self.having,
            order_by: self.order_by,
            limit: self.limit,
        }
    }
}

impl<Select, From, Where, GroupBy, Having, OrderBy, Limit> ToQuery
    for SelectStatement<Select, From, Where, GroupBy, Having, OrderBy, Limit>
where
    Select: traits::SelectExpr,
    From: traits::FromExpr,
    Where: traits::WhereExpr,
    GroupBy: traits::GroupByExpr,
    Having: traits::HavingExpr,
    OrderBy: traits::OrderByExpr,
    Limit: traits::LimitExpr,
{
    fn write<W: std::io::prelude::Write>(
        &self,
        stream: &mut W,
        ctx: &mut crate::ToQueryContext,
    ) -> Result<(), std::io::Error> {
        write!(stream, "SELECT ");
        todo!()
    }
}

#[derive(Default, Clone, Copy)]
/// Wildcard (*)
pub struct All {}
pub const ALL: All = All {};
impl SelectExpr for All {}
impl<T> SelectExpr for T where T: traits::Term {}
impl<T> SelectExpr for Alias<T> where T: traits::Term {}

/// A serie of select expressions.
/// Work recursively.
pub struct SelectExprChain<S1, S2>(pub S1, pub S2)
where
    S1: SelectExpr,
    S2: SelectExpr;

#[cfg(test)]
mod tests {
    use super::{SelectStatement, ALL};

    #[test]
    fn test_select_with_from() {
        SelectStatement::new(ALL);
    }
}
