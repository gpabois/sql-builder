use sql_builder_macros::BooleanPrimary;

use crate::grammar as G;
use crate::Database;
use crate::ToQuery;

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

impl<SearchCond> ::std::fmt::Display for NestedSearchCondition<SearchCond>
where
    SearchCond: G::SearchCondition + ::std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({})", self.0)
    }
}

impl<DB, SearchCond> ToQuery<DB> for NestedSearchCondition<SearchCond>
where
    DB: Database,
    SearchCond: G::SearchCondition + ToQuery<DB>,
{
    fn write<W: std::io::Write>(
        &self,
        stream: &mut W,
        ctx: &mut crate::ToQueryContext<DB>,
    ) -> Result<(), std::io::Error> {
        write!(stream, "(")?;
        self.0.write(stream, ctx)?;
        write!(stream, ")")
    }
}
