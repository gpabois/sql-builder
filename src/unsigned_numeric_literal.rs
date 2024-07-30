use crate::Database;
use crate::ToQuery;
use sql_builder_macros::UnsignedNumericLiteral;
use std::fmt::Write;

#[derive(Clone, Copy, UnsignedNumericLiteral)]
pub enum UnsignedNumericLiteral {
    Int(u64),
    Float(f64),
}

impl std::fmt::Display for UnsignedNumericLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UnsignedNumericLiteral::Int(val) => write!(f, "{}", val),
            UnsignedNumericLiteral::Float(val) => write!(f, "{}", val),
        }
    }
}

impl<'q, DB> ToQuery<'q, DB> for UnsignedNumericLiteral
where
    DB: Database,
{
    fn write(&'q self, ctx: &mut crate::ToQueryContext<'q, DB>) -> std::fmt::Result {
        write!(ctx, "{}", self)
    }
}

impl From<u64> for UnsignedNumericLiteral {
    fn from(value: u64) -> Self {
        Self::Int(value)
    }
}

impl From<f64> for UnsignedNumericLiteral {
    fn from(value: f64) -> Self {
        Self::Float(value.abs())
    }
}

#[inline]
pub fn unsigned_numeric_lit<V>(value: V) -> UnsignedNumericLiteral
where
    UnsignedNumericLiteral: From<V>,
{
    UnsignedNumericLiteral::from(value)
}
