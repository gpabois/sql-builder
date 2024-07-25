use sql_builder_macros::BooleanPrimary;

use crate::grammar as G;
use crate::helpers as H;
use crate::{grammar::SearchCondition, ToQuery};

#[derive(BooleanPrimary)]
pub struct NestedSearchCondition<Cond>(pub(crate) Cond)
where
    Cond: G::SearchCondition;

impl<Cond> NestedSearchCondition<Cond>
where
    Cond: G::SearchCondition,
{
    pub fn new(cond: Cond) -> Self {
        Self(cond)
    }
}

impl<SearchCond> ToQuery for NestedSearchCondition<SearchCond>
where
    SearchCond: SearchCondition,
{
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
