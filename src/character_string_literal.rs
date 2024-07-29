use sql_builder_macros::CharacterStringLiteral;

use crate::{Database, ToQuery};

#[derive(CharacterStringLiteral)]
pub struct CharacterStringLiteralRef<'a>(&'a str);

impl ::std::fmt::Display for CharacterStringLiteralRef<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut prev = '\0';

        write!(f, "'")?;
        for c in self.0.chars() {
            if c == '\'' && prev != '\'' && prev != '\0' {
                write!(f, "''")?;
            } else {
                write!(f, "{}", c)?;
            }

            prev = c;
        }
        write!(f, "'")?;

        Ok(())
    }
}

impl<DB> ToQuery<DB> for CharacterStringLiteralRef<'_>
where
    DB: Database,
{
    /// Write the string literal
    /// Autoescape quote
    fn write<W: std::io::Write>(
        &self,
        stream: &mut W,
        _ctx: &mut crate::ToQueryContext<DB>,
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

