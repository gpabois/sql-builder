use crate::{traits, ToQuery};

impl traits::FromClause for () {
    const IS_IMPL: bool = false;
}

pub struct FromClause<TableRef: traits::TableReference> {
    table_ref: TableRef,
}

impl<T> From<T> for FromClause<T>
where
    T: traits::TableReference,
{
    fn from(table_ref: T) -> Self {
        Self { table_ref }
    }
}

impl<TableRef> ToQuery for FromClause<TableRef>
where
    TableRef: traits::TableReference,
{
    fn write<W: std::io::prelude::Write>(
        &self,
        stream: &mut W,
        ctx: &mut crate::ToQueryContext,
    ) -> Result<(), std::io::Error> {
        write!(stream, "FROM ")?;
        self.table_ref.write(stream, ctx)
    }
}

impl<TableRef> traits::FromClause for FromClause<TableRef>
where
    TableRef: traits::TableReference,
{
    const IS_IMPL: bool = true;
}

#[cfg(test)]
mod test {
    use super::FromClause;
    use crate::{identifier::id, ToQuery};

    #[test]
    fn test_from_identifier() {
        let clause = FromClause::from(id("my_table"));
        let sql = clause.to_string().unwrap();
        assert_eq!(sql, "FROM my_table");
    }
}

