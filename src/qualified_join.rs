use crate::{
    grammar::{self as G},
    join_condition::JoinCondition,
    named_columns_join::NamedColumnsJoin,
    ToQuery,
};
use sql_builder_macros::QualifiedJoin;
use sqlx::Database;
use std::fmt::Write;

#[derive(Clone, Copy, QualifiedJoin)]
pub struct QualifiedJoin<Ref, Kind, Primary, Spec>
where
    Ref: G::TableReference,
    Primary: G::TablePrimary,
    Kind: G::JoinType,
    Spec: G::JoinSpecification,
{
    table_src: Ref,
    table_dest: Primary,
    kind: Kind,
    spec: Spec,
}

impl<Src, Kind, Dest, Spec> QualifiedJoin<Src, Kind, Dest, Spec>
where
    Src: G::TableReference,
    Dest: G::TablePrimary,
    Kind: G::JoinType,
    Spec: G::JoinSpecification,
{
    pub fn new(table_src: Src, table_dest: Dest, kind: Kind, spec: Spec) -> Self {
        Self {
            table_src,
            table_dest,
            kind,
            spec,
        }
    }
}

impl<Src, Kind, Dest, Spec> std::fmt::Display for QualifiedJoin<Src, Kind, Dest, Spec>
where
    Src: G::TableReference + std::fmt::Display,
    Dest: G::TablePrimary + std::fmt::Display,
    Kind: G::JoinType + std::fmt::Display,
    Spec: G::JoinSpecification + std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.table_src.fmt(f)?;
        write!(f, " ")?;

        if <Kind as G::JoinType>::IS_IMPL {
            write!(f, "{} ", self.kind)?;
        }

        write!(f, "JOIN {} {}", self.table_dest, self.spec)
    }
}

impl<'q, DB, Src, Kind, Dest, Spec> ToQuery<'q, DB> for QualifiedJoin<Src, Kind, Dest, Spec>
where
    DB: Database,
    Src: G::TableReference + ToQuery<'q, DB>,
    Dest: G::TablePrimary + ToQuery<'q, DB>,
    Kind: G::JoinType + ToQuery<'q, DB>,
    Spec: G::JoinSpecification + ToQuery<'q, DB>,
{
    fn write(&'q self, ctx: &mut crate::ToQueryContext<'q, DB>) -> std::fmt::Result {
        self.table_src.write(ctx)?;
        write!(ctx, " ")?;

        if <Kind as G::JoinType>::IS_IMPL {
            self.kind.write(ctx)?;
            write!(ctx, " ")?;
        }

        write!(ctx, "JOIN ")?;
        self.table_dest.write(ctx)?;
        write!(ctx, " ")?;
        self.spec.write(ctx)
    }
}

pub struct QualifiedJoinFragment<Src, Dest, Kind>
where
    Src: G::TableReference,
    Dest: G::TablePrimary,
    Kind: G::JoinType,
{
    table_src: Src,
    table_dest: Dest,
    kind: Kind,
}

/// A partial fragment to build a qualified join between two tables.
impl<Src, Dest, Kind> QualifiedJoinFragment<Src, Dest, Kind>
where
    Src: G::TableReference,
    Dest: G::TablePrimary,
    Kind: G::JoinType,
{
    pub fn new(table_src: Src, table_dest: Dest, kind: Kind) -> Self {
        Self {
            table_src,
            table_dest,
            kind,
        }
    }

    pub fn on<Cond>(self, cond: Cond) -> QualifiedJoin<Src, Kind, Dest, JoinCondition<Cond>>
    where
        Cond: G::SearchCondition,
    {
        QualifiedJoin::new(
            self.table_src,
            self.table_dest,
            self.kind,
            JoinCondition::new(cond),
        )
    }

    pub fn using<Cols>(
        self,
        columns: Cols,
    ) -> QualifiedJoin<Src, Kind, Dest, NamedColumnsJoin<Cols>>
    where
        Cols: G::JoinColumnList,
    {
        QualifiedJoin::new(
            self.table_src,
            self.table_dest,
            self.kind,
            NamedColumnsJoin::new(columns),
        )
    }
}
