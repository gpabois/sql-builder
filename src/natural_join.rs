use crate::{grammar as G, ToQuery};
use sql_builder_macros::NaturalJoin;
use sqlx::Database;
use std::fmt::Write;

#[derive(Clone, Copy, NaturalJoin)]
pub struct NaturalJoin<Src, Kind, Dest>
where
    Src: G::TableReference,
    Kind: G::JoinType,
    Dest: G::TablePrimary,
{
    table_src: Src,
    table_dest: Dest,
    kind: Kind,
}

impl<Src, Kind, Dest> std::fmt::Display for NaturalJoin<Src, Kind, Dest>
where
    Src: G::TableReference + std::fmt::Display,
    Kind: G::JoinType + std::fmt::Display,
    Dest: G::TablePrimary + std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} NATURAL ", self.table_src)?;

        if <Kind as G::JoinType>::IS_IMPL {
            write!(f, "{} ", self.kind)?;
        }

        write!(f, "JOIN {}", self.table_dest)
    }
}

impl<'q, DB, Src, Kind, Dest> ToQuery<'q, DB> for NaturalJoin<Src, Kind, Dest>
where
    DB: Database,
    Src: G::TableReference + ToQuery<'q, DB>,
    Kind: G::JoinType + ToQuery<'q, DB>,
    Dest: G::TablePrimary + ToQuery<'q, DB>,
{
    fn write(&'q self, ctx: &mut crate::ToQueryContext<'q, DB>) -> std::fmt::Result {
        self.table_src.write(ctx)?;
        write!(ctx, " NATURAL ")?;

        if <Kind as G::JoinType>::IS_IMPL {
            self.kind.write(ctx)?;
            write!(ctx, " ")?;
        }

        write!(ctx, "JOIN ")?;
        self.table_dest.write(ctx)
    }
}
