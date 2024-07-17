use sql_builder_macros::BooleanFactor;

use crate::{traits, ToQuery};

#[derive(BooleanFactor)]
pub struct Not<BoolTest>(pub(crate) BoolTest) where BoolTest: traits::BooleanTest;

impl<BoolTest> ToQuery for Not<BoolTest> where BoolTest: traits::BooleanTest {
    fn write<W: std::io::Write>(
        &self,
        stream: &mut W,
        ctx: &mut crate::ToQueryContext,
    ) -> Result<(), std::io::Error> {
        todo!()
    }
} 