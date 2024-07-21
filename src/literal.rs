use sql_builder_macros::Literal;

use crate::{grammar::Literal, ToQuery};

#[derive(Literal)]
/// A literal
pub enum Lit {
    String(String),
    Long(i64),
    UnsignedLong(u64),
    Float(f32),
    Double(f64),
}

impl From<&str> for Lit {
    fn from(value: &str) -> Self {
        Self::String(value.to_owned())
    }
}

impl From<i64> for Lit {
    fn from(value: i64) -> Self {
        Self::Long(value)
    }
}

impl From<u64> for Lit {
    fn from(value: u64) -> Self {
        Self::UnsignedLong(value)
    }
}

impl From<f32> for Lit {
    fn from(value: f32) -> Self {
        Self::Float(value)
    }
}

impl From<f64> for Lit {
    fn from(value: f64) -> Self {
        Self::Double(value)
    }
}

impl ToQuery for Lit {
    fn write<W: std::io::Write>(
        &self,
        stream: &mut W,
        _ctx: &mut crate::ToQueryContext,
    ) -> Result<(), std::io::Error> {
        match self {
            Lit::String(val) => write!(stream, "{}", val),
            Lit::Long(val) => write!(stream, "{}", val),
            Lit::UnsignedLong(val) => write!(stream, "{}", val),
            Lit::Float(val) => write!(stream, "{}", val),
            Lit::Double(val) => write!(stream, "{}", val),
        }
    }
}

/// Creates a literal.
pub fn lit<V>(value: V) -> impl Literal
where
    Lit: From<V>,
{
    Lit::from(value)
}
