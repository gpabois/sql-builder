use crate::{grammar as G, ToQuery};
use sql_builder_macros::CrossJoin;
use std::fmt::Write;

#[derive(Clone, Copy, CrossJoin)]
pub struct CrossJoin<Lhs, Rhs>
where
    Lhs: G::TableReference,
    Rhs: G::TablePrimary,
{
    lhs: Lhs,
    rhs: Rhs,
}

impl<Lhs, Rhs> CrossJoin<Lhs, Rhs>
where
    Lhs: G::TableReference,
    Rhs: G::TablePrimary,
{
    pub fn new(lhs: Lhs, rhs: Rhs) -> Self {
        Self { lhs, rhs }
    }
}

impl<Lhs, Rhs> std::fmt::Display for CrossJoin<Lhs, Rhs>
where
    Lhs: G::TableReference + std::fmt::Display,
    Rhs: G::TablePrimary + std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} CROSS JOIN {}", self.lhs, self.rhs)
    }
}

impl<'q, DB, Lhs, Rhs> ToQuery<'q, DB> for CrossJoin<Lhs, Rhs>
where
    DB: ::sqlx::Database,
    Lhs: G::TableReference + ToQuery<'q, DB>,
    Rhs: G::TablePrimary + ToQuery<'q, DB>,
{
    fn write(&'q self, ctx: &mut crate::ToQueryContext<'q, DB>) -> std::fmt::Result {
        self.lhs.write(ctx)?;
        write!(ctx, " CROSS JOIN ")?;
        self.rhs.write(ctx)
    }
}
