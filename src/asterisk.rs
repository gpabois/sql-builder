use sql_builder_macros::Asterisk;

use crate::ToQuery;

#[derive(Asterisk)]
/// Asterisk (*)
pub struct Asterisk;
impl ToQuery for Asterisk {
    fn write<W: std::io::Write>(
        &self,
        stream: &mut W,
        _ctx: &mut crate::ToQueryContext,
    ) -> Result<(), std::io::Error> {
        write!(stream, "*")
    }
}