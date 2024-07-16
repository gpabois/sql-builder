use crate::traits;

impl<T> traits::Condition for T where T: traits::Term {}

pub struct And<C1, C2>(C1, C2)
where
    C1: traits::Condition,
    C2: traits::Condition;

impl<C1, C2> traits::Condition for And<C1, C2>
where
    C1: traits::Condition,
    C2: traits::Condition,
{
}

pub struct Or<C1, C2>(C1, C2)
where
    C1: traits::Condition,
    C2: traits::Condition;

impl<C1, C2> traits::Condition for Or<C1, C2>
where
    C1: traits::Condition,
    C2: traits::Condition,
{
}

pub struct Not<C>(C)
where
    C: traits::Condition;

impl<C> traits::Condition for Not<C> where C: traits::Condition {}
