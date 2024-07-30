use sql_builder_macros::DynamicParameterSpecification;

use crate::ToQuery;

#[derive(DynamicParameterSpecification)]
/// A bound parameter
pub struct Bound<T> {
    param: T,
}

impl<T> Bound<T> {
    pub fn new(param: T) -> Self {
        Self { param }
    }
}

impl<T> std::fmt::Display for Bound<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "?")
    }
}

impl<'q, DB, T> ToQuery<'q, DB> for Bound<T>
where
    DB: ::sqlx::Database,
    T: ::sqlx::Encode<'q, DB> + ::sqlx::Type<DB>,
{
    fn write(&'q self, ctx: &mut crate::ToQueryContext<'q, DB>) -> std::fmt::Result {
        ctx.write_argument(&self.param)
    }
}

pub fn bind<T>(value: T) -> Bound<T> {
    Bound::new(value)
}
