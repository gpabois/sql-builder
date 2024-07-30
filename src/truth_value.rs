use crate::Database;
use crate::ToQuery;
use sql_builder_macros::TruthValue;
use std::fmt::Write;

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

impl<'q, DB> ToQuery<'q, DB> for True
where
    DB: Database,
{
    fn write(&'q self, ctx: &mut crate::ToQueryContext<'q, DB>) -> std::fmt::Result {
        write!(ctx, "{}", self)
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

impl<'q, DB> ToQuery<'q, DB> for False
where
    DB: Database,
{
    fn write(&'q self, ctx: &mut crate::ToQueryContext<'q, DB>) -> std::fmt::Result {
        write!(ctx, "{}", self)
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

impl<'q, DB> ToQuery<'q, DB> for Unknown
where
    DB: Database,
{
    fn write(&'q self, ctx: &mut crate::ToQueryContext<'q, DB>) -> std::fmt::Result {
        write!(ctx, "{}", self)
    }
}
