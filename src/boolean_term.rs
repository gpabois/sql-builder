use sql_builder_macros::BooleanTerm;

use crate::{traits, ToQuery};

#[derive(BooleanTerm)]
pub struct And<BoolTerm, BoolFactor>(BoolTerm, BoolFactor)
where
    BoolTerm: traits::BooleanTerm,
    BoolFactor: traits::BooleanFactor;

impl<BoolTerm, BoolFactor> ToQuery for And<BoolTerm, BoolFactor>
where
    BoolTerm: traits::BooleanTerm,
    BoolFactor: traits::BooleanFactor,
{
    fn write<W: std::io::Write>(
        &self,
        stream: &mut W,
        ctx: &mut crate::ToQueryContext,
    ) -> Result<(), std::io::Error> {
        todo!()
    }
}

pub fn and<BoolTerm, BoolFactor>(lhs: BoolTerm, rhs: BoolFactor) -> And<BoolTerm, BoolFactor>
where
    BoolTerm: traits::BooleanTerm,
    BoolFactor: traits::BooleanFactor,
{
    And(lhs, rhs)
}

