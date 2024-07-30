use crate::{Database, Symbol, ToQuery, ToQueryContext};
use sql_builder_macros::Blank;

#[derive(Clone, Copy, Blank)]
/// Blank type for default symbol trait implementation.
pub struct Blank;

impl Symbol for Blank {}

impl<'q, DB> ToQuery<'q, DB> for Blank
where
    DB: Database,
{
    fn write(&'q self, _ctx: &mut ToQueryContext<DB>) -> ::std::fmt::Result {
        Ok(())
    }
}

impl ::std::fmt::Display for Blank {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "")
    }
}
