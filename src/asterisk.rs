use sql_builder_macros::Asterisk;

use crate::{Database, ToQuery};

#[derive(Asterisk)]
/// Asterisk (*)
pub struct Asterisk;

impl<DB> ToQuery<DB> for Asterisk
where
    DB: Database,
{
    fn write<W: std::io::Write>(
        &self,
        stream: &mut W,
        _ctx: &mut crate::ToQueryContext<DB>,
    ) -> Result<(), std::io::Error> {
        write!(stream, "*")
    }
}

impl std::fmt::Display for Asterisk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "*")
    }
}
