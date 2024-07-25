use sql_builder_macros::QuerySpecification;

use crate::{from_clause::From, ToQuery};

use crate::grammar as G;
use crate::helpers as H;

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
/// ```ignore
/// use sql_builder::{select, id};
///
/// let selected_columns = id!(col1)
/// .add_selection(id!(col2).alias_column(id!(aliased_column)))
/// .add_selection(id!(col3));
/// let table = id!(my_table);
/// 
/// let stmt = select(selected_columns).from(table);
///
/// let sql = sel.to_raw_query().unwrap();
/// assert_eq!(sql, "SELECT col1, col2 AS aliased_column, col3 FROM my_table");
/// ```
pub fn select<Selection: G::SelectList>(select_list: Selection) -> BeginSelect<Selection> {
    BeginSelect { select_list }
}

/// Creates an incomplete select beginning.
///
/// To get a valid select statement, [self::BeginSelect::from] must be used.
/// See [self::select]
pub struct BeginSelect<SeLs>
where
    SeLs: G::SelectList,
{
    select_list: SeLs,
}

impl<SeLs> BeginSelect<SeLs>
where
    SeLs: G::SelectList,
{
    pub fn from<TabRefs>(self, table_refs: TabRefs) -> impl G::QuerySpecification
    where
        TabRefs: G::TableReferenceList,
    {
        Select {
            quantifier: None,
            select_list: self.select_list,
            table_expression: From::new(table_refs),
        }
    }
}

#[derive(QuerySpecification)]
/// Represents a select statement.
/// See [self::select]
pub struct Select<SeLs, TabExpr>
where
    TabExpr: G::TableExpression,
    SeLs: G::SelectList,
{
    quantifier: Option<SetQuantifier>,
    select_list: SeLs,
    table_expression: TabExpr,
}

impl<SeLs, TabExpr> H::QuerySpecification for Select<SeLs, TabExpr>
where
    SeLs: G::SelectList,
    TabExpr: G::TableExpression,
{
    type TableExpression = TabExpr;

    #[inline]
    fn distinct(self) -> impl G::QuerySpecification {
        Select {
            quantifier: Some(SetQuantifier::Distinct),
            select_list: self.select_list,
            table_expression: self.table_expression,
        }
    }

    #[inline]
    fn all(self) -> impl G::QuerySpecification {
        Select {
            quantifier: Some(SetQuantifier::All),
            select_list: self.select_list,
            table_expression: self.table_expression,
        }
    }

    fn transform_table_expression<NewTableExpr>(
        self,
        transform: impl FnOnce(Self::TableExpression) -> NewTableExpr,
    ) -> impl G::QuerySpecification
    where
        NewTableExpr: G::TableExpression,
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
    SeLs: G::SelectList,
    TabExpr: G::TableExpression,
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