use sql_builder_macros::QuerySpecification;

use crate::either::Either;
use crate::{from_clause::From, ToQuery};

use crate::grammar as G;
use crate::helpers as H;
use crate::Database;

/// The select quantifier, either ALL or DISTINCT.
/// See [self::Select::distinct] or [self::Select::all]
pub enum SetQuantifier {
    All,
    Distinct,
}

impl AsRef<str> for SetQuantifier {
    fn as_ref(&self) -> &str {
        match self {
            SetQuantifier::All => "ALL",
            SetQuantifier::Distinct => "DISTINC",
        }
    }
}

impl std::fmt::Display for SetQuantifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
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
    pub fn from<TabRefs>(self, table_refs: TabRefs) -> Select<SeLs, From<TabRefs>>
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
    pub quantifier: Option<SetQuantifier>,
    pub select_list: SeLs,
    pub table_expression: TabExpr,
}

impl<Selection, Table> H::QuerySpecification for Select<Selection, Table>
where
    Selection: G::SelectList,
    Table: G::TableExpression,
{
    type SelectList = Selection;
    type TableExpression = Table;

    #[inline]
    fn unwrap(self) -> Self {
        self
    }
}

impl<Selection, Table> std::fmt::Display for Select<Selection, Table>
where
    Selection: G::SelectList + std::fmt::Display,
    Table: G::TableExpression + std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SELECT ")?;

        match &self.quantifier {
            Some(q) => write!(f, "{} ", q)?,
            None => {}
        };

        write!(f, "{} {}", self.select_list, self.table_expression)
    }
}
impl<DB, Selection, Table> ToQuery<DB> for Select<Selection, Table>
where
    DB: Database,
    Selection: G::SelectList + ToQuery<DB>,
    Table: G::TableExpression + ToQuery<DB>,
{
    fn write<W: std::io::Write>(
        &self,
        stream: &mut W,
        ctx: &mut crate::ToQueryContext<DB>,
    ) -> Result<(), std::io::Error> {
        write!(stream, "SELECT ")?;

        match &self.quantifier {
            Some(q) => write!(stream, "{}", q)?,
            None => {}
        };

        self.select_list.write(stream, ctx)?;
        write!(stream, " ")?;
        self.table_expression.write(stream, ctx)
    }
}

impl<Lhs, Rhs> H::QuerySpecification for Either<Lhs, Rhs>
where
    Lhs: G::QuerySpecification,
    Rhs: G::QuerySpecification,
{
    type SelectList = Either<Lhs::SelectList, Rhs::SelectList>;
    type TableExpression = Either<Lhs::TableExpression, Rhs::TableExpression>;

    fn unwrap(self) -> Select<Self::SelectList, Self::TableExpression> {
        match self {
            Either::Left(lhs) => {
                let Select {
                    quantifier,
                    select_list,
                    table_expression,
                } = lhs.unwrap();
                Select {
                    quantifier,
                    select_list: Either::Left(select_list),
                    table_expression: Either::Left(table_expression),
                }
            }
            Either::Right(rhs) => {
                let Select {
                    quantifier,
                    select_list,
                    table_expression,
                } = rhs.unwrap();

                Select {
                    quantifier,
                    select_list: Either::Right(select_list),
                    table_expression: Either::Right(table_expression),
                }
            }
        }
    }
}
