use sql_builder_macros::{SignedNumericLiteral, UnsignedNumericLiteral};

use crate::ToQuery;


#[derive(SignedNumericLiteral)]
pub enum SignedNumericLiteral {
    Int(i64),
    Float(f64)
}

impl ToQuery for SignedNumericLiteral {
    fn write<W: std::io::Write>(
        &self,
        stream: &mut W,
        ctx: &mut crate::ToQueryContext,
    ) -> Result<(), std::io::Error> {
        match self {
            Self::Int(val) => write!(stream, "{}", val),
            Self::Float(val) => write!(stream, "{}", val),
        }
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
pub fn signed_numeric_lit<V>(value: V) -> SignedNumericLiteral where SignedNumericLiteral: From<V> {
    SignedNumericLiteral::from(value)
}