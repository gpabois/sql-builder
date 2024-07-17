use crate::{traits, ToQuery};

pub struct Add<Lhs, Rhs>(Lhs, Rhs) where Lhs: traits::Term, Rhs: traits::Term;
pub struct Sub<Lhs, Rhs>(Lhs, Rhs) where Lhs: traits::Term, Rhs: traits::Term;

impl<Lhs, Rhs> traits::Term for Add<Lhs, Rhs> where Lhs: traits::Term, Rhs: traits::Term {}
impl<Lhs, Rhs> traits::NumericValueExpression for Add<Lhs, Rhs> where Lhs: traits::Term, Rhs: traits::Term {}
impl<Lhs, Rhs> traits::ValueExpression for Add<Lhs, Rhs> where Lhs: traits::Term, Rhs: traits::Term {}
impl<Lhs, Rhs> traits::DerivedColumn for Add<Lhs, Rhs> where Lhs: traits::Term, Rhs: traits::Term {}
impl<Lhs, Rhs> traits::SelectList for Add<Lhs, Rhs> where Lhs: traits::Term, Rhs: traits::Term {
    const IS_IMPL: bool = true;
}
impl<Lhs, Rhs> ToQuery for Add<Lhs, Rhs> where Lhs: traits::Term, Rhs: traits::Term {
    fn write<W: std::io::Write>(
        &self,
        stream: &mut W,
        ctx: &mut crate::ToQueryContext,
    ) -> Result<(), std::io::Error> {
        todo!()
    }
}

impl<Lhs, Rhs> traits::Term for Sub<Lhs, Rhs> where Lhs: traits::Term, Rhs: traits::Term {}
impl<Lhs, Rhs> traits::NumericValueExpression for Sub<Lhs, Rhs> where Lhs: traits::Term, Rhs: traits::Term {}
impl<Lhs, Rhs> traits::ValueExpression for Sub<Lhs, Rhs> where Lhs: traits::Term, Rhs: traits::Term {}
impl<Lhs, Rhs> traits::DerivedColumn for Sub<Lhs, Rhs> where Lhs: traits::Term, Rhs: traits::Term {}
impl<Lhs, Rhs> traits::SelectList for Sub<Lhs, Rhs> where Lhs: traits::Term, Rhs: traits::Term {
    const IS_IMPL: bool = true;
}
impl<Lhs, Rhs> ToQuery for Sub<Lhs, Rhs> where Lhs: traits::Term, Rhs: traits::Term {
    fn write<W: std::io::Write>(
        &self,
        stream: &mut W,
        ctx: &mut crate::ToQueryContext,
    ) -> Result<(), std::io::Error> {
        todo!()
    }
}

pub fn add<Lhs, Rhs>(lhs: Lhs, rhs: Rhs) -> Add<Lhs, Rhs> where Lhs: traits::Term, Rhs: traits::Term {
    Add(lhs, rhs)
}

pub fn sub<Lhs, Rhs>(lhs: Lhs, rhs: Rhs) -> Sub<Lhs, Rhs> where Lhs: traits::Term, Rhs: traits::Term {
    Sub(lhs, rhs)
}