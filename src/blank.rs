use crate::{Symbol, ToQuery, ToQueryContext};
use sql_builder_macros::Blank;
use std::io::Write;

#[derive(Blank)]
/// Blank type for default symbol trait implementation.
pub struct Blank;

impl Symbol for Blank {}

impl ToQuery for Blank {
    fn write<W: Write>(
        &self,
        _stream: &mut W,
        _ctx: &mut ToQueryContext,
    ) -> Result<(), std::io::Error> {
        Ok(())
    }
}
