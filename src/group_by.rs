use crate::traits::{self, Expression};

pub struct GroupByExpr<E>(E)
where
    E: Expression;

impl<E> traits::GroupByExpr for GroupByExpr<E> where E: Expression {}
