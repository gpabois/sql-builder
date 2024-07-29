use sql_builder_macros::SignedNumericLiteral;

use crate::Database;
use crate::ToQuery;

#[derive(SignedNumericLiteral)]
pub enum SignedNumericLiteral {
    Int(i64),
    Float(f64),
}

impl ::std::fmt::Display for SignedNumericLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Int(val) => write!(f, "{}", val),
            Self::Float(val) => write!(f, "{}", val),
        }
    }
}

impl<DB> ToQuery<DB> for SignedNumericLiteral
where
    DB: Database,
{
    fn write<W: std::io::Write>(
        &self,
        stream: &mut W,
        _ctx: &mut crate::ToQueryContext<DB>,
    ) -> Result<(), std::io::Error> {
        write!(stream, "{}", self)
    }
}

impl From<i64> for SignedNumericLiteral {
    fn from(value: i64) -> Self {
        Self::Int(value)
    }
}

impl From<f64> for SignedNumericLiteral {
    fn from(value: f64) -> Self {
        Self::Float(value)
    }
}

#[inline]
pub fn signed_numeric_lit<V>(value: V) -> SignedNumericLiteral
where
    SignedNumericLiteral: From<V>,
{
    SignedNumericLiteral::from(value)
}

