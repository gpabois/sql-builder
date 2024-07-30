use crate::{
    blank::Blank,
    either::Either,
    from_constructor::FromConstructor,
    grammar::{self, InsertColumnsAndSources, InsertionTarget, OverrideClause},
    ToQuery,
};
use sql_builder_macros::Insert;
use std::fmt::Write;

use crate::grammar as G;
use crate::helpers as H;
use crate::Database;

#[derive(Insert)]
pub struct Insert<Target, Values>
where
    Target: G::InsertionTarget,
    Values: G::InsertColumnsAndSources,
{
    target: Target,
    values: Values,
}

impl<Target, Values> H::Insert for Insert<Target, Values>
where
    Target: InsertionTarget,
    Values: InsertColumnsAndSources,
{
    type Target = Target;
    type ColumnsAndSources = Values;

    fn transform_target<NewTarget: InsertionTarget>(
        self,
        transform: impl FnOnce(Self::Target) -> NewTarget,
    ) -> impl grammar::Insert {
        Insert {
            target: transform(self.target),
            values: self.values,
        }
    }

    /// Transform the columns and sources
    fn transform_columns_and_sources<NewColumnsAndSources: InsertColumnsAndSources>(
        self,
        transform: impl FnOnce(Self::ColumnsAndSources) -> NewColumnsAndSources,
    ) -> impl grammar::Insert {
        Insert {
            target: self.target,
            values: transform(self.values),
        }
    }
}

impl<Target, Values> std::fmt::Display for Insert<Target, Values>
where
    Target: InsertionTarget + std::fmt::Display,
    Values: InsertColumnsAndSources + std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "INSERT INTO {} {}", self.target, self.values)
    }
}

impl<'q, DB, Target, Values> ToQuery<'q, DB> for Insert<Target, Values>
where
    DB: Database,
    Target: InsertionTarget + ToQuery<'q, DB>,
    Values: InsertColumnsAndSources + ToQuery<'q, DB>,
{
    fn write(&'q self, ctx: &mut crate::ToQueryContext<'q, DB>) -> std::fmt::Result {
        write!(ctx, "INSERT INTO ")?;
        self.target.write(ctx)?;
        write!(ctx, " ")?;
        self.values.write(ctx)
    }
}

impl<Lhs, Rhs> H::Insert for Either<Lhs, Rhs>
where
    Lhs: G::Insert,
    Rhs: G::Insert,
{
    type Target = Either<Lhs::Target, Rhs::Target>;
    type ColumnsAndSources = Either<Lhs::ColumnsAndSources, Rhs::ColumnsAndSources>;

    fn transform_target<NewTarget: grammar::InsertionTarget>(
        self,
        transform: impl FnOnce(Self::Target) -> NewTarget,
    ) -> impl grammar::Insert {
        self.apply_with_args(
            transform,
            |lhs, transform| lhs.transform_target(|a| transform(Either::Left(a))),
            |rhs, transform| rhs.transform_target(|a| transform(Either::Right(a))),
        )
    }

    fn transform_columns_and_sources<NewColumnsAndSources: grammar::InsertColumnsAndSources>(
        self,
        transform: impl FnOnce(Self::ColumnsAndSources) -> NewColumnsAndSources,
    ) -> impl grammar::Insert {
        self.apply_with_args(
            transform,
            |lhs, transform| lhs.transform_columns_and_sources(|a| transform(Either::Left(a))),
            |rhs, transform| rhs.transform_columns_and_sources(|a| transform(Either::Right(a))),
        )
    }
}

/// Begin an insert command
pub struct InsertFragment<Target>
where
    Target: G::InsertionTarget,
{
    target: Target,
}

impl<Target> InsertFragment<Target>
where
    Target: G::InsertionTarget,
{
    pub fn columns<Columns>(
        self,
        columns: Columns,
    ) -> InsertFromConstructorFragment<Target, Blank, Columns>
    where
        Columns: G::ColumnNameList,
    {
        InsertFromConstructorFragment {
            target: self.target,
            override_clause: Blank,
            columns,
        }
    }
}

pub struct InsertFromConstructorFragment<Target, Override, Columns>
where
    Target: G::InsertionTarget,
    Columns: G::ColumnNameList,
    Override: G::OverrideClause,
{
    target: Target,
    columns: Columns,
    override_clause: Override,
}

impl<Target, Override, Columns> InsertFromConstructorFragment<Target, Override, Columns>
where
    Target: G::InsertionTarget,
    Columns: G::ColumnNameList,
    Override: G::OverrideClause,
{
    pub fn r#override<NewOverride: OverrideClause>(
        self,
        override_clause: NewOverride,
    ) -> InsertFromConstructorFragment<Target, NewOverride, Columns> {
        InsertFromConstructorFragment {
            target: self.target,
            columns: self.columns,
            override_clause,
        }
    }

    /// Set the values to be inserted in the table
    ///
    /// This creates a valid insert command.
    pub fn values<Value>(
        self,
        values: Value,
    ) -> Insert<Target, FromConstructor<Columns, Override, Value>>
    where
        Value: G::ContextuallyTypedTableValueConstructor,
    {
        Insert {
            target: self.target,
            values: FromConstructor::new(self.columns, self.override_clause, values),
        }
    }
}

#[inline]
/// Creates an insertion statement.
pub fn insert<Target>(target: Target) -> InsertFragment<Target>
where
    Target: G::InsertionTarget,
{
    InsertFragment { target }
}
