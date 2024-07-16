pub mod alias;
pub mod condition;
pub mod error;
pub mod expression;
pub mod factor;
pub mod from;
pub mod group_by;
pub mod name;
pub mod select;
pub mod table;
pub mod term;
pub mod r#where;

use std::io::Write;

#[derive(Default)]
pub struct ToQueryContext {}

pub trait ToQuery {
    fn write<W: Write>(
        &self,
        stream: &mut W,
        ctx: &mut ToQueryContext,
    ) -> Result<(), std::io::Error>;
}

impl ToQuery for () {
    fn write<W: Write>(
        &self,
        _stream: &mut W,
        _ctx: &mut ToQueryContext,
    ) -> Result<(), std::io::Error> {
        Ok(())
    }
}

pub mod traits {
    use crate::{alias::Alias, name::Name, select::SelectExprChain};

    pub trait SelectExpr: Sized {
        /// Chain a new select expression
        fn chain<T: SelectExpr, N: Into<T>>(self, next: N) -> SelectExprChain<Self, T> {
            SelectExprChain(self, next.into())
        }
    }

    impl SelectExpr for () {}

    pub trait WhereExpr {
        const IS_IMPL: bool;
    }

    pub trait FromExpr {
        const IS_IMPL: bool;
    }
    pub trait HavingExpr {}
    impl HavingExpr for () {}

    pub trait GroupByExpr {}
    impl GroupByExpr for () {}

    pub trait OrderByExpr {}
    impl OrderByExpr for () {}

    pub trait LimitExpr {}
    impl LimitExpr for () {}

    pub trait Expression {}
    pub trait Condition {}

    impl<T> Expression for T where T: Condition {}

    pub trait Term: Sized {
        fn alias<N: Into<Name>>(self, name: N) -> Alias<Self> {
            Alias(self, name.into())
        }
    }
}
