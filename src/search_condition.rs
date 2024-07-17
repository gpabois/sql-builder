use sql_builder_macros::SearchCondition;
use crate::{traits, ToQuery};

#[derive(SearchCondition)]
pub struct Or<SearchCond, BoolTerm>(SearchCond, BoolTerm) 
    where SearchCond: traits::SearchCondition, 
          BoolTerm: traits::BooleanTerm;

impl<SearchCond, BoolTerm> ToQuery for Or<SearchCond, BoolTerm> 
where SearchCond: traits::SearchCondition, 
        BoolTerm: traits::BooleanTerm
{
    fn write<W: std::io::Write>(
        &self,
        stream: &mut W,
        ctx: &mut crate::ToQueryContext,
    ) -> Result<(), std::io::Error> {
        todo!()
    }
}

