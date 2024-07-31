use sql_builder_macros::JoinType;
use sqlx::Database;
use std::fmt::Write;

use crate::ToQuery;

#[derive(JoinType)]
pub struct Inner;

impl AsRef<str> for Inner {
    fn as_ref(&self) -> &str {
        "INNER"
    }
}

impl std::fmt::Display for Inner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

impl<'q, DB> ToQuery<'q, DB> for Inner
where
    DB: Database,
{
    fn write(&'q self, ctx: &mut crate::ToQueryContext<'q, DB>) -> std::fmt::Result {
        write!(ctx, "{}", self.as_ref())
    }
}

#[derive(JoinType)]
pub struct Left;

impl AsRef<str> for Left {
    fn as_ref(&self) -> &str {
        "LEFT"
    }
}

impl std::fmt::Display for Left {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

impl<'q, DB> ToQuery<'q, DB> for Left
where
    DB: Database,
{
    fn write(&'q self, ctx: &mut crate::ToQueryContext<'q, DB>) -> std::fmt::Result {
        write!(ctx, "{}", self.as_ref())
    }
}

#[derive(JoinType)]
pub struct Right;

impl AsRef<str> for Right {
    fn as_ref(&self) -> &str {
        "RIGHT"
    }
}

impl std::fmt::Display for Right {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

impl<'q, DB> ToQuery<'q, DB> for Right
where
    DB: Database,
{
    fn write(&'q self, ctx: &mut crate::ToQueryContext<'q, DB>) -> std::fmt::Result {
        write!(ctx, "{}", self.as_ref())
    }
}

#[derive(JoinType)]
pub struct LeftOuter;

impl AsRef<str> for LeftOuter {
    fn as_ref(&self) -> &str {
        "LEFT OUTER"
    }
}

impl std::fmt::Display for LeftOuter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

impl<'q, DB> ToQuery<'q, DB> for LeftOuter
where
    DB: Database,
{
    fn write(&'q self, ctx: &mut crate::ToQueryContext<'q, DB>) -> std::fmt::Result {
        write!(ctx, "{}", self.as_ref())
    }
}

#[derive(JoinType)]
pub struct RightOuter;

impl AsRef<str> for RightOuter {
    fn as_ref(&self) -> &str {
        "RIGHT OUTER"
    }
}

impl std::fmt::Display for RightOuter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

impl<'q, DB> ToQuery<'q, DB> for RightOuter
where
    DB: Database,
{
    fn write(&'q self, ctx: &mut crate::ToQueryContext<'q, DB>) -> std::fmt::Result {
        write!(ctx, "{}", self.as_ref())
    }
}
