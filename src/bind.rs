use sqlx::{Database, Encode};

use crate::ToQuery;

/// A bound parameter
///
/// Is <term>
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

impl<DB, T> ToQuery<DB> for Bound<T>
where
    DB: Database,
    for<'r> T: Encode<'r, DB>,
{
    fn write<W: std::io::prelude::Write>(
        &self,
        stream: &mut W,
        ctx: &mut crate::ToQueryContext<DB>,
    ) -> Result<(), std::io::Error> {
        todo!()
    }
}

fn bind<T>(value: T) -> Bound<T> {
    Bound::new(value)
}
