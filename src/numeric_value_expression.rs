use sql_builder_macros::NumericValueExpression;

use crate::grammar as G;
use crate::{
    grammar::{NumericValueExpression, Term},
    Database, ToQuery,
};

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

impl<DB, Lhs, Rhs> ToQuery<DB> for ArithmOperand<Lhs, Rhs>
where
    DB: Database,
    Lhs: G::NumericValueExpression + ToQuery<DB>,
    Rhs: G::Term + ToQuery<DB>,
{
    fn write<W: std::io::Write>(
        &self,
        stream: &mut W,
        ctx: &mut crate::ToQueryContext<DB>,
    ) -> Result<(), std::io::Error> {
        self.lhs.write(stream, ctx)?;
        write!(stream, " {} ", self.kind)?;
        self.rhs.write(stream, ctx)
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
