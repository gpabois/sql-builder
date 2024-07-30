use crate::{Database, ToQuery};
use sql_builder_macros::CharacterStringLiteral;
use std::fmt::Write;

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

impl<'q, DB> ToQuery<'q, DB> for CharacterStringLiteralRef<'_>
where
    DB: Database,
{
    /// Write the string literal
    /// Autoescape quote
    fn write(&'q self, ctx: &mut crate::ToQueryContext<'q, DB>) -> std::fmt::Result {
        let mut prev = '\0';

        write!(ctx, "'")?;
        for c in self.0.chars() {
            if c == '\'' && prev != '\'' && prev != '\0' {
                write!(ctx, "''")?;
            } else {
                write!(ctx, "{}", c)?;
            }

            prev = c;
        }
        write!(ctx, "'")?;

        Ok(())
    }
}

pub fn char_str_lit(value: &str) -> CharacterStringLiteralRef<'_> {
    CharacterStringLiteralRef(value)
}
