use crate::ToQuery;

pub enum Either<Lhs, Rhs> {
    Left(Lhs),
    Right(Rhs),
}

impl<Lhs, Rhs> Either<Lhs, Rhs> {
    pub fn if_else(predicate: bool, lhs: Lhs, f: impl FnOnce(Lhs) -> Rhs) -> Self {
        if predicate {
            Self::Left(lhs)
        } else {
            Self::Right(f(lhs))
        }
    }
}

impl<Lhs, Rhs> ToQuery for Either<Lhs, Rhs>
where
    Lhs: ToQuery,
    Rhs: ToQuery,
{
    fn write<W: std::io::prelude::Write>(
        &self,
        stream: &mut W,
        ctx: &mut crate::ToQueryContext,
    ) -> Result<(), std::io::Error> {
        match self {
            Either::Left(left) => left.write(stream, ctx),
            Either::Right(right) => right.write(stream, ctx),
        }
    }
}
