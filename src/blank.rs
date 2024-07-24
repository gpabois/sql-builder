use std::io::Write;
use sql_builder_macros::Blank;
use crate::{ToQuery, ToQueryContext};

#[derive(Blank)]
/// Blank type for default symbol trait implementation.
pub struct Blank;

impl ToQuery for Blank {
    fn write<W: Write>(
        &self,
        _stream: &mut W,
        _ctx: &mut ToQueryContext,
    ) -> Result<(), std::io::Error> {
        Ok(())
    }
}
