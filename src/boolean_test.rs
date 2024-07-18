use sql_builder_macros::BooleanTest;

use crate::{traits, ToQuery};

#[derive(BooleanTest)]
pub struct IsTruthValue<BoolPrimary, TruthValue>
where
    BoolPrimary: traits::BooleanPrimary,
    TruthValue: traits::TruthValue,
{
    pub(crate) lhs: BoolPrimary,
    pub(crate) rhs: TruthValue,
}

impl<BoolPrimary, TruthValue> ToQuery for IsTruthValue<BoolPrimary, TruthValue>
where
    BoolPrimary: traits::BooleanPrimary,
    TruthValue: traits::TruthValue,
{
    fn write<W: std::io::Write>(
        &self,
        stream: &mut W,
        ctx: &mut crate::ToQueryContext,
    ) -> Result<(), std::io::Error> {
        todo!()
    }
}

#[derive(BooleanTest)]
pub struct IsNotTruthValue<BoolPrimary, TruthValue>
where
    BoolPrimary: traits::BooleanPrimary,
    TruthValue: traits::TruthValue,
{
    pub(crate) lhs: BoolPrimary,
    pub(crate) rhs: TruthValue,
}

impl<BoolPrimary, TruthValue> ToQuery for IsNotTruthValue<BoolPrimary, TruthValue>
where
    BoolPrimary: traits::BooleanPrimary,
    TruthValue: traits::TruthValue,
{
    fn write<W: std::io::Write>(
        &self,
        stream: &mut W,
        ctx: &mut crate::ToQueryContext,
    ) -> Result<(), std::io::Error> {
        todo!()
    }
}
