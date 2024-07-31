use crate::{grammar as G, ToQuery};
use sql_builder_macros::UnionJoin;
use sqlx::Database;
use std::fmt::Write;

#[derive(Clone, Copy, UnionJoin)]
pub struct UnionJoin<Src, Dest>
where
    Src: G::TableReference,
    Dest: G::TablePrimary,
{
    table_src: Src,
    table_dest: Dest,
}

impl<Src, Dest> std::fmt::Display for UnionJoin<Src, Dest>
where
    Src: G::TableReference + std::fmt::Display,
    Dest: G::TablePrimary + std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} UNION JOIN {}", self.table_src, self.table_dest)
    }
}

impl<'q, DB, Src, Dest> ToQuery<'q, DB> for UnionJoin<Src, Dest>
where
    DB: Database,
    Src: G::TableReference + ToQuery<'q, DB>,
    Dest: G::TablePrimary + ToQuery<'q, DB>,
{
    fn write(&'q self, ctx: &mut crate::ToQueryContext<'q, DB>) -> std::fmt::Result {
        self.table_src.write(ctx)?;
        write!(ctx, " UNION JOIN ")?;
        self.table_dest.write(ctx)
    }
}
