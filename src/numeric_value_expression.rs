use crate::grammar as G;
use crate::{
    grammar::{NumericValueExpression, Term},
    Database, ToQuery,
};
use sql_builder_macros::NumericValueExpression;
use std::fmt::Write;

enum ArithmOpKind {
    Add,
    Sub,
}

impl AsRef<str> for ArithmOpKind {
    fn as_ref(&self) -> &str {
        match self {
            ArithmOpKind::Add => "+",
            ArithmOpKind::Sub => "-",
        }
    }
}

impl std::fmt::Display for ArithmOpKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

#[derive(NumericValueExpression)]
pub struct ArithmOperand<Lhs, Rhs>
where
    Lhs: G::NumericValueExpression,
    Rhs: G::Term,
{
    lhs: Lhs,
    rhs: Rhs,
    kind: ArithmOpKind,
}

impl<Lhs, Rhs> std::fmt::Display for ArithmOperand<Lhs, Rhs>
where
    Lhs: G::NumericValueExpression + std::fmt::Display,
    Rhs: G::Term + std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.lhs, self.kind, self.rhs)
    }
}

impl<'q, DB, Lhs, Rhs> ToQuery<'q, DB> for ArithmOperand<Lhs, Rhs>
where
    DB: Database,
    Lhs: G::NumericValueExpression + ToQuery<'q, DB>,
    Rhs: G::Term + ToQuery<'q, DB>,
{
    fn write(&'q self, ctx: &mut crate::ToQueryContext<'q, DB>) -> std::fmt::Result {
        self.lhs.write(ctx)?;
        write!(ctx, " {} ", self.kind)?;
        self.rhs.write(ctx)
    }
}

#[inline]
pub fn add<Lhs, Rhs>(
    lhs: impl NumericValueExpression,
    rhs: impl Term,
) -> impl NumericValueExpression {
    ArithmOperand {
        lhs,
        rhs,
        kind: ArithmOpKind::Add,
    }
}

#[inline]
pub fn sub<Lhs, Rhs>(
    lhs: impl NumericValueExpression,
    rhs: impl Term,
) -> impl NumericValueExpression {
    ArithmOperand {
        lhs,
        rhs,
        kind: ArithmOpKind::Sub,
    }
}
