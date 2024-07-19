use crate::{grammar, ToQuery};
use sql_builder_macros::SelectList;

/// A list of select expressions.
/// Work recursively.
#[derive(SelectList)]
pub struct SelectLink<S1, S2>(pub S1, pub S2)
where
    S1: grammar::SelectList,
    S2: grammar::SelectList;

impl<S1, S2> ToQuery for SelectLink<S1, S2>
where
    S1: grammar::SelectList,
    S2: grammar::SelectList,
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
