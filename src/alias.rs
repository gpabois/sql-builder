use crate::{name::Name, traits};

pub struct Alias<T: traits::Term>(pub T, pub Name);
