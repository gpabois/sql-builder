use ::std::fmt::Display;

use sql_builder_macros::Either;

use crate::Database;
use crate::ToQuery;

#[derive(Clone, Copy, Either)]
pub enum Either<Lhs, Rhs> {
    Left(Lhs),
    Right(Rhs),
}

impl<Lhs, Rhs> Either<Lhs, Rhs> {
    #[inline]
    pub fn if_else(predicate: bool, lhs: Lhs, f: impl FnOnce(Lhs) -> Rhs) -> Self {
        if predicate {
            Self::Left(lhs)
        } else {
            Self::Right(f(lhs))
        }
    }

    #[inline]
    pub fn apply<NewLhs, NewRhs>(
        self,
        on_left: impl FnOnce(Lhs) -> NewLhs,
        on_right: impl FnOnce(Rhs) -> NewRhs,
    ) -> Either<NewLhs, NewRhs> {
        match self {
            Either::Left(lhs) => Either::Left(on_left(lhs)),
            Either::Right(rhs) => Either::Right(on_right(rhs)),
        }
    }

    #[inline]
    pub fn apply_with_args<NewLhs, NewRhs, Args>(
        self,
        args: Args,
        on_left: impl FnOnce(Lhs, Args) -> NewLhs,
        on_right: impl FnOnce(Rhs, Args) -> NewRhs,
    ) -> Either<NewLhs, NewRhs> {
        match self {
            Either::Left(lhs) => Either::Left(on_left(lhs, args)),
            Either::Right(rhs) => Either::Right(on_right(rhs, args)),
        }
    }
}

impl<Lhs, Rhs> Display for Either<Lhs, Rhs>
where
    Lhs: std::fmt::Display,
    Rhs: std::fmt::Display,
{
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Either::Left(left) => left.fmt(f),
            Either::Right(right) => right.fmt(f),
        }
    }
}

impl<'q, DB, Lhs, Rhs> ToQuery<'q, DB> for Either<Lhs, Rhs>
where
    DB: Database,
    Lhs: ToQuery<'q, DB>,
    Rhs: ToQuery<'q, DB>,
{
    fn write(&'q self, ctx: &mut crate::ToQueryContext<'q, DB>) -> ::std::fmt::Result {
        match self {
            Either::Left(left) => left.write(ctx),
            Either::Right(right) => right.write(ctx),
        }
    }
}
