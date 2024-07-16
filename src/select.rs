use crate::{
    alias::Alias,
    traits::{self, FromExpr, SelectExpr},
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
    pub fn new<E: SelectExpr, F: Into<E>>(select: F) -> InitSelectStatement<E> {
        InitSelectStatement {
            kind: SelectKind::Default,
            select: select.into(),
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
    pub fn from<E: FromExpr, F: Into<E>>(self, where: F) -> SelectStatement<Select, E, Where, GroupBy, Having, OrderBy, Limit> {
        Self {
            kind: self.kind,
            select: self.select,
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

/// Wildcard (*)
pub struct All {}
impl SelectExpr for All {}
impl<T> SelectExpr for T where T: traits::Term {}
impl<T> SelectExpr for Alias<T> where T: traits::Term {}

/// A serie of select expressions.
/// Work recursively.
pub struct SelectExprChain<S1, S2>(pub S1, pub S2)
where
    S1: SelectExpr,
    S2: SelectExpr;
