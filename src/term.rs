use sql_builder_macros::Term;

use crate::ToQuery;

use crate::grammar as G;
use crate::helpers as H;

enum TermOperandKind {
    Mult,
    Div,
}

impl AsRef<str> for TermOperandKind {
    fn as_ref(&self) -> &str {
        match self {
            TermOperandKind::Mult => "*",
            TermOperandKind::Div => "/",
        }
    }
}

impl ToQuery for TermOperandKind {
    fn write<W: std::io::Write>(
        &self,
        stream: &mut W,
        _ctx: &mut crate::ToQueryContext,
    ) -> Result<(), std::io::Error> {
        write!(stream, "{}", self.as_ref())
    }
}

#[derive(Term)]
pub struct TermOperand<Lhs, Rhs>
where
    Lhs: G::Term,
    Rhs: G::Factor,
{
    lhs: Lhs,
    kind: TermOperandKind,
    rhs: Rhs,
}

impl<Lhs, Rhs> H::SelectSublist for TermOperand<Lhs, Rhs>
where
    Lhs: G::Term,
    Rhs: G::Factor,
{
}

impl<Lhs, Rhs> ToQuery for TermOperand<Lhs, Rhs>
where
    Lhs: G::Term,
    Rhs: G::Factor,
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
pub fn mult(lhs: impl G::Term, rhs: impl G::Factor) -> impl G::Term {
    TermOperand {
        lhs,
        rhs,
        kind: TermOperandKind::Mult,
    }
}

#[inline]
pub fn div(lhs: impl G::Term, rhs: impl G::Factor) -> impl G::Term {
    TermOperand {
        lhs,
        rhs,
        kind: TermOperandKind::Div,
    }
}
