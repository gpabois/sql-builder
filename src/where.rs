use crate::{traits, ToQuery};

/// WHERE <search_condition>
pub struct WhereClause<SearchCond: traits::SearchCondition>{
    search_condition: SearchCond
}

impl<SearchCond> From<SearchCond> for WhereClause<SearchCond> where SearchCond: traits::SearchCondition {
    fn from(search_condition: SearchCond) -> Self {
        Self { search_condition }
    }
}

impl<SearchCond> traits::WhereClause for WhereClause<SearchCond> where SearchCond: traits::SearchCondition
{
    const IS_IMPL: bool = true;
}

impl<SearchCond> ToQuery for WhereClause<SearchCond> where SearchCond: traits::SearchCondition
{
    fn write<W: std::io::Write>(
        &self,
        stream: &mut W,
        ctx: &mut crate::ToQueryContext,
    ) -> Result<(), std::io::Error> {
        write!(stream, "FROM ")?;
        self.search_condition.write(stream, ctx)
    }
}