use crate::traits;
use sqlx::{Database, Encode};
use std::{marker::PhantomData, ops::Deref};

/// A bound parameter
pub struct Bound<DB: Database, P>
where
    for<'r> P: Encode<'r, DB>,
{
    param: P,
    _pht: PhantomData<DB>,
}

impl<DB: Database, P> traits::Term for Bound<DB, P> where for<'r> P: Encode<'r, DB> {}

/// A string value
pub struct Str(String);
impl From<&str> for Str {
    fn from(value: &str) -> Self {
        Self(value.to_owned())
    }
}
impl From<String> for Str {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl traits::Term for Str {}

pub struct Int(i32);
impl From<i32> for Int {
    #[inline]
    fn from(value: i32) -> Self {
        Self(value)
    }
}
impl traits::Term for Int {}

pub struct Long(i64);
impl From<i64> for Long {
    fn from(value: i64) -> Self {
        Self(value)
    }
}
impl traits::Term for Long {}

pub struct Decimal(f64);
impl From<f64> for Decimal {
    fn from(value: f64) -> Self {
        Self(value)
    }
}
impl traits::Term for Decimal {}
