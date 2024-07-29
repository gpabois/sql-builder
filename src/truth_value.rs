use sql_builder_macros::TruthValue;

use crate::Database;
use crate::ToQuery;

#[derive(TruthValue)]
pub struct True;

impl AsRef<str> for True {
    fn as_ref(&self) -> &str {
        "TRUE"
    }
}
impl std::fmt::Display for True {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

impl<DB> ToQuery<DB> for True
where
    DB: Database,
{
    fn write<W: std::io::Write>(
        &self,
        stream: &mut W,
        _ctx: &mut crate::ToQueryContext<DB>,
    ) -> Result<(), std::io::Error> {
        write!(stream, "{}", self)
    }
}

#[derive(TruthValue)]
pub struct False;

impl AsRef<str> for False {
    fn as_ref(&self) -> &str {
        "FALSE"
    }
}

impl std::fmt::Display for False {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

impl<DB> ToQuery<DB> for False
where
    DB: Database,
{
    fn write<W: std::io::Write>(
        &self,
        stream: &mut W,
        _ctx: &mut crate::ToQueryContext<DB>,
    ) -> Result<(), std::io::Error> {
        write!(stream, "{}", self)
    }
}

#[derive(TruthValue)]
pub struct Unknown;

impl AsRef<str> for Unknown {
    fn as_ref(&self) -> &str {
        "UNKNOWN"
    }
}

impl std::fmt::Display for Unknown {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

impl<DB> ToQuery<DB> for Unknown
where
    DB: Database,
{
    fn write<W: std::io::Write>(
        &self,
        stream: &mut W,
        _ctx: &mut crate::ToQueryContext<DB>,
    ) -> Result<(), std::io::Error> {
        write!(stream, "{}", self)
    }
}
