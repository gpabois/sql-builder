use crate::{grammar::HavingClause, Blank};

impl HavingClause for Blank {
    const IS_IMPL: bool = false;
}
