use sql_builder_macros::BooleanPrimary;

use crate::{grammar::SearchCondition, ToQuery};

#[derive(BooleanPrimary)]
pub struct NestedSearchCondition<SearchCond>(pub(crate) SearchCond) where SearchCond: SearchCondition;

impl<SearchCond> ToQuery for NestedSearchCondition<SearchCond> where SearchCond: SearchCondition {
    fn write<W: std::io::Write>(
        &self,
        stream: &mut W,
        ctx: &mut crate::ToQueryContext,
    ) -> Result<(), std::io::Error> {
        write!(stream, "(")?;
        self.0.write(stream, ctx)?;
        write!(stream, ")")
    }
}