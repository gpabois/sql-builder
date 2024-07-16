use crate::{name::Name, ToQuery};

pub struct TableExpr {
    pub schema_name: Option<Name>,
    pub table_name: Name,
    pub alias: Option<Name>,
}

impl ToQuery for TableExpr {
    fn write<W: std::io::prelude::Write>(
        &self,
        stream: &mut W,
        ctx: &mut crate::ToQueryContext,
    ) -> Result<(), std::io::Error> {
        if let Some(schema_name) = &self.schema_name {
            schema_name.write(stream, ctx)?;
            write!(stream, ".")?;
        }

        self.table_name.write(stream, ctx)?;

        if let Some(alias) = &self.alias {
            write!(stream, " AS ")?;
            alias.write(stream, ctx)?;
        }

        Ok(())
    }
}
