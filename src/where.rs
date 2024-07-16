use crate::traits;

impl traits::WhereExpr for () {
    const IS_IMPL: bool = false;
}

pub struct WhereExpr<E>(E)
where
    E: traits::Expression;

impl<E> traits::WhereExpr for WhereExpr<E>
where
    E: traits::Expression,
{
    const IS_IMPL: bool = true;
}
