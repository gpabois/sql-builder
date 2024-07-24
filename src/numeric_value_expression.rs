use sql_builder_macros::NumericValueExpression;

use crate::{
    grammar::{NumericValueExpression, Term},
    ToQuery,
};
use crate::grammar as G;
use crate::helpers as H;

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

impl ToQuery for ArithmOpKind {
    fn write<W: std::io::Write>(
        &self,
        stream: &mut W,
        _ctx: &mut crate::ToQueryContext,
    ) -> Result<(), std::io::Error> {
        write!(stream, "{}", self.as_ref())
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

impl<Lhs, Rhs> H::SelectSublist for ArithmOperand<Lhs, Rhs>
where
    Lhs: G::NumericValueExpression,
    Rhs: G::Term,
{}

impl<Lhs, Rhs> ToQuery for ArithmOperand<Lhs, Rhs>
where
    Lhs: G::NumericValueExpression,
    Rhs: G::Term,
{
    fn write<W: std::io::Write>(
        &self,
        stream: &mut W,
        ctx: &mut crate::ToQueryContext,
    ) -> Result<(), std::io::Error> {
        self.lhs.write(stream, ctx)?;
        write!(stream, " ")?;
        self.kind.write(stream, ctx)?;
        write!(stream, " ")?;
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
