use sql_builder_macros::TruthValue;

use crate::ToQuery;

#[derive(TruthValue)]
pub struct True;

impl ToQuery for True {
    fn write<W: std::io::Write>(
        &self,
        stream: &mut W,
        _ctx: &mut crate::ToQueryContext,
    ) -> Result<(), std::io::Error> {
        write!(stream, "TRUE")
    }
}

#[derive(TruthValue)]
pub struct False;

impl ToQuery for False {
    fn write<W: std::io::Write>(
        &self,
        stream: &mut W,
        _ctx: &mut crate::ToQueryContext,
    ) -> Result<(), std::io::Error> {
        write!(stream, "FALSE")
    }
}

#[derive(TruthValue)]
pub struct Unknown;

impl ToQuery for Unknown {
    fn write<W: std::io::Write>(
        &self,
        stream: &mut W,
        _ctx: &mut crate::ToQueryContext,
    ) -> Result<(), std::io::Error> {
        write!(stream, "UNKNOWN")
    }
}

