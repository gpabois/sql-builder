use crate::{Database, Symbol, ToQuery, ToQueryContext};
use sql_builder_macros::Blank;
use std::io::Write;

#[derive(Blank)]
/// Blank type for default symbol trait implementation.
pub struct Blank;

impl Symbol for Blank {}

impl<DB> ToQuery<DB> for Blank
where
    DB: Database,
{
    fn write<W: Write>(
        &self,
        _stream: &mut W,
        _ctx: &mut ToQueryContext<DB>,
    ) -> Result<(), std::io::Error> {
        Ok(())
    }
}

impl ::std::fmt::Display for Blank {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "")
    }
}
