use sql_builder_macros::SQLArgumentList;
use sqlx::Database;
use std::fmt::Write;

use crate::{grammar as G, ToQuery, ToQueryContext};

#[derive(Clone, Copy, SQLArgumentList)]
pub struct SQLArgumentLink<Head, Tail>
where
    Head: G::SQLArgumentList,
    Tail: G::SQLArgument,
{
    head: Head,
    tail: Tail,
}

impl<Head, Tail> SQLArgumentLink<Head, Tail>
where
    Head: G::SQLArgumentList,
    Tail: G::SQLArgument,
{
    pub fn new(head: Head, tail: Tail) -> Self {
        Self { head, tail }
    }
}

impl<Head, Tail> ::std::fmt::Display for SQLArgumentLink<Head, Tail>
where
    Head: G::SQLArgumentList + std::fmt::Display,
    Tail: G::SQLArgument + std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}", self.head, self.tail)
    }
}

impl<'q, DB, Head, Tail> ToQuery<'q, DB> for SQLArgumentLink<Head, Tail>
where
    DB: Database,
    Head: G::SQLArgumentList + ToQuery<'q, DB>,
    Tail: G::SQLArgument + ToQuery<'q, DB>,
{
    fn write(&'q self, ctx: &mut ToQueryContext<'q, DB>) -> std::fmt::Result {
        self.head.write(ctx)?;
        write!(ctx, ", ")?;
        self.tail.write(ctx)
    }
}
