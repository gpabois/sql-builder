use sql_builder_macros::UnsignedNumericLiteral;

use crate::{ToQuery, helpers as H};


#[derive(UnsignedNumericLiteral)]
pub enum UnsignedNumericLiteral {
    Int(u64),
    Float(f64)
}

impl H::SelectSublist for UnsignedNumericLiteral {}
impl H::ValueExpression for UnsignedNumericLiteral {}

impl ToQuery for UnsignedNumericLiteral {
    fn write<W: std::io::Write>(
        &self,
        stream: &mut W,
        ctx: &mut crate::ToQueryContext,
    ) -> Result<(), std::io::Error> {
        match self {
            UnsignedNumericLiteral::Int(val) => write!(stream, "{}", val),
            UnsignedNumericLiteral::Float(val) => write!(stream, "{}", val),
        }
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
pub fn unsigned_numeric_lit<V>(value: V) -> UnsignedNumericLiteral where UnsignedNumericLiteral: From<V> {
    UnsignedNumericLiteral::from(value)
}