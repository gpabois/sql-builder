use crate::{Database, ToQuery};
use sql_builder_macros::Asterisk;
use std::fmt::Write;

#[derive(Asterisk)]
/// Asterisk (*)
pub struct Asterisk;

impl<'q, DB> ToQuery<'q, DB> for Asterisk
where
    DB: Database,
{
    fn write(&'q self, stream: &mut crate::ToQueryContext<DB>) -> std::fmt::Result {
        write!(stream, "*")
    }
}

impl std::fmt::Display for Asterisk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "*")
    }
}
