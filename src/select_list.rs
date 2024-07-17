use sql_builder_macros::SelectList;
use crate::{traits, ToQuery};

/// A list of select expressions.
/// Work recursively.
#[derive(SelectList)]
pub struct SelectList<S1, S2>(pub S1, pub S2)
where
    S1: traits::SelectList,
    S2: traits::SelectList;

impl<S1,S2> ToQuery for SelectList<S1, S2> 
where
    S1: traits::SelectList,
    S2: traits::SelectList
{
    fn write<W: std::io::Write>(
        &self,
        stream: &mut W,
        ctx: &mut crate::ToQueryContext,
    ) -> Result<(), std::io::Error> {
        self.0.write(stream, ctx)?;
        write!(stream, ", ")?;
        self.1.write(stream, ctx)
    }
}