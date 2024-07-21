use sql_builder_macros::SelectList;

use crate::{
    from_clause::From,
    grammar::{self, SelectList, TableExpression, TableReferenceList},
    ToQuery,
};

/// The select quantifier, either ALL or DISTINCT.
/// See [self::Select::distinct] or [self::Select::all]
pub enum SetQuantifier {
    All,
    Distinct,
}

#[inline]
/// Creates a select statement
///
/// # Example
/// ```
/// use sql_builder::{select, id};
///
/// let selected_columns = id("col1")
/// .chain(id("col2").alias(id("aliased_column")))
/// .chain(id("col3"));
///
/// let sel = select(selected_columns).from(id("my_table"));
///
/// let sql = sel.to_string().unwrap();
/// assert_eq!(sql, "SELECT col1, col2 AS aliased_column, col3 FROM my_table");
/// ```
pub fn select<SeLs: grammar::SelectList>(select_list: SeLs) -> BeginSelect<SeLs> {
    BeginSelect { select_list }
}

/// Creates an incomplete select beginning.
///
/// To get a valid select statement, [self::BeginSelect::from] must be used.
/// See [self::select]
pub struct BeginSelect<SeLs>
where
    SeLs: SelectList,
{
    select_list: SeLs,
}

impl<SeLs> BeginSelect<SeLs>
where
    SeLs: SelectList,
{
    pub fn from<TabRefs: TableReferenceList>(
        self,
        table_refs: TabRefs,
    ) -> Select<SeLs, From<TabRefs>> {
        Select {
            quantifier: None,
            select_list: self.select_list,
            table_expression: From::new(table_refs),
        }
    }
}

/// Represents a select statement.
/// See [self::select]
pub struct Select<SeLs, TabExpr>
where
    TabExpr: TableExpression,
    SeLs: SelectList,
{
    quantifier: Option<SetQuantifier>,
    select_list: SeLs,
    table_expression: TabExpr,
}

impl<SeLs, TabExpr> grammar::Select for Select<SeLs, TabExpr>
where
    SeLs: SelectList,
    TabExpr: TableExpression,
{
    type TableExpr = TabExpr;

    #[inline]
    fn distinct(self) -> impl grammar::Select {
        Select {
            quantifier: Some(SetQuantifier::Distinct),
            select_list: self.select_list,
            table_expression: self.table_expression,
        }
    }

    #[inline]
    fn all(self) -> impl grammar::Select {
        Select {
            quantifier: Some(SetQuantifier::All),
            select_list: self.select_list,
            table_expression: self.table_expression,
        }
    }

    fn transform_table_expression<NewTableExpr>(
        self,
        transform: impl FnOnce(Self::TableExpr) -> NewTableExpr,
    ) -> impl grammar::Select
    where
        NewTableExpr: TableExpression,
    {
        Select {
            quantifier: self.quantifier,
            select_list: self.select_list,
            table_expression: transform(self.table_expression),
        }
    }
}

impl<SeLs, TabExpr> ToQuery for Select<SeLs, TabExpr>
where
    SeLs: SelectList,
    TabExpr: TableExpression,
{
    fn write<W: std::io::Write>(
        &self,
        stream: &mut W,
        ctx: &mut crate::ToQueryContext,
    ) -> Result<(), std::io::Error> {
        write!(stream, "SELECT ")?;

        match self.quantifier {
            Some(SetQuantifier::All) => write!(stream, "ALL ")?,
            Some(SetQuantifier::Distinct) => write!(stream, "DISTINCT ")?,
            None => {}
        };

        self.select_list.write(stream, ctx)?;
        write!(stream, " ")?;
        self.table_expression.write(stream, ctx)
    }
}

#[derive(Clone, Copy, SelectList)]
/// Wildcard (*)
pub struct All {}
pub const ALL: All = All {};
impl ToQuery for All {
    fn write<W: std::io::Write>(
        &self,
        stream: &mut W,
        _ctx: &mut crate::ToQueryContext,
    ) -> Result<(), std::io::Error> {
        write!(stream, "*")
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        grammar::{DerivedColumn, Select, SelectList},
        id,
        select::select,
        ToQuery as _,
    };

    #[test]
    fn test_select_basic() {
        let selected_columns = id("col1")
            .append(id("col2").alias(id("aliased_column")))
            .append(id("col3"));

        let stmt = select(selected_columns).from(id("my_table"));

        let sql = stmt.to_string().unwrap();
        assert_eq!(
            sql,
            "SELECT col1, col2 AS aliased_column, col3 FROM my_table"
        );
    }

    #[test]
    fn test_select_distinct() {
        let sel = select(id("col1")).from(id("my_table")).distinct();
        let sql = sel.to_string().unwrap();
        assert_eq!(sql, "SELECT DISTINCT col1 FROM my_table");
    }
}
