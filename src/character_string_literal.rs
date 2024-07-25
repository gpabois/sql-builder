use sql_builder_macros::CharacterStringLiteral;

use crate::ToQuery;

#[derive(CharacterStringLiteral)]
pub struct CharacterStringLiteralRef<'a>(&'a str);

impl ToQuery for CharacterStringLiteralRef<'_> {
    /// Write the string literal
    /// Autoescape quote
    fn write<W: std::io::Write>(
        &self,
        stream: &mut W,
        _ctx: &mut crate::ToQueryContext,
    ) -> Result<(), std::io::Error> {
        let mut prev = '\0';

        write!(stream, "'")?;
        for c in self.0.chars() {
            if c == '\'' && prev != '\'' && prev != '\0' {
                write!(stream, "''")?;
            } else {
                write!(stream, "{}", c)?;
            }

            prev = c;
        }
        write!(stream, "'")?;

        Ok(())
    }
}

pub fn char_str_lit(value: &str) -> CharacterStringLiteralRef<'_> {
    CharacterStringLiteralRef(value)
}