use sql_builder_macros::Term;

use crate::grammar as G;
use crate::Database;
use crate::ToQuery;

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

impl std::fmt::Display for TermOperandKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
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

impl<Lhs, Rhs> ::std::fmt::Display for TermOperand<Lhs, Rhs>
where
    Lhs: G::Term + std::fmt::Display,
    Rhs: G::Factor + std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.lhs, self.kind, self.rhs)
    }
}
impl<DB, Lhs, Rhs> ToQuery<DB> for TermOperand<Lhs, Rhs>
where
    DB: Database,
    Lhs: G::Term + ToQuery<DB>,
    Rhs: G::Factor + ToQuery<DB>,
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
