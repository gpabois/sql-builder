use crate::traits;
use sqlx::{Database, Encode};
use std::marker::PhantomData;

/// A bound parameter
/// 
/// Is <term>
pub struct Bound<DB: Database, P>
where
    for<'r> P: Encode<'r, DB>,
{
    param: P,
    _pht: PhantomData<DB>,
}

impl<DB, P> Bound<DB, P> where DB: Database, for<'r> P: Encode<'r, DB> {
    pub fn new(param: P) -> Self {
        Self {
            param,
            _pht: PhantomData
        }
    }
}

fn bind<DB: Database, P>(value: P) -> Bound<DB, P> where for<'r> P: Encode<'r, DB> {
    Bound::new(value)
}