//! SQL Builder
//!
//! A crate to build SQL queries that comply with the ISO/IEC 9075-2:2003, at compile-time.
//!
//! # Notes
//! Currently, only a fragment of the standard is implemented.
//!
//! The current development roadmap is to allow to build most of the *simple* queries
//! (direct SELECT, INSERT, UPDATE and DELETE commands).
//!
//! # How to build a SELECT query
//! ```ignore
//! use sql_builder::{select, id, prelude::*};
//!
//! let selected_columns = id!(col1)
//! .add_selection(id!(col2).alias_column(id!(aliased_column)))
//! .add_selection(id!(col3));
//! let table = id!(my_table);
//!
//! let stmt = select(selected_columns).from(table);
//!
//! let sql = sel.to_raw_query().unwrap();
//! assert_eq!(sql, "SELECT col1, col2 AS aliased_column, col3 FROM my_table");
//! ```
//!
//! # How to build an INSERT query
//!
pub mod error;

pub mod group_by;
pub mod select;
pub mod select_sublist;
pub mod table_expression;
pub mod table_reference_list;

pub mod bind;
pub mod identifier;
pub mod numeric_value_expression;
pub mod term;
pub mod where_clause;

pub mod derived_column;
pub mod from_clause;

pub mod asterisk;
pub mod blank;
pub mod boolean_factor;
pub mod boolean_primary;
pub mod boolean_term;
pub mod boolean_test;
pub mod character_string_literal;
pub mod column_name_list;
pub mod comparison_predicate;
pub mod contextually_typed_row_value_constructor;
pub mod contextually_typed_row_value_constructor_element_list;
pub mod contextually_typed_row_value_expression_list;
pub mod cross_join;
pub mod either;
pub mod from_constructor;
pub mod having_clause;
pub mod identifier_chain;
pub mod insert;
pub mod join_condition;
pub mod join_type;
pub mod named_columns_join;
pub mod natural_join;
pub mod qualified_join;
pub mod schema_name;
pub mod search_condition;
pub mod signed_numeric_literal;
pub mod truth_value;
pub mod union_join;
pub mod unqualified_schema_name;
pub mod unsigned_numeric_literal;

use sqlx::Arguments as _;
pub use sqlx::Database;
use std::marker::PhantomData;

pub struct ToQueryContext<'q, DB>
where
    DB: ::sqlx::Database,
{
    args: <DB as ::sqlx::Database>::Arguments<'q>,
    sql: String,
    _pht: PhantomData<DB>,
}

impl<'q, DB> std::fmt::Write for ToQueryContext<'q, DB>
where
    DB: ::sqlx::Database,
{
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        self.sql.write_str(s)
    }
}

impl<'q, DB> Default for ToQueryContext<'q, DB>
where
    DB: Database,
{
    fn default() -> Self {
        Self {
            _pht: PhantomData,
            args: Default::default(),
            sql: String::default(),
        }
    }
}

impl<'q, DB> ToQueryContext<'q, DB>
where
    DB: Database,
{
    pub fn write_argument<T>(&mut self, value: &'q T) -> std::fmt::Result
    where
        T: ::sqlx::Encode<'q, DB> + ::sqlx::Type<DB>,
    {
        self.args
            .add(value)
            .unwrap_or_else(|_| panic!("cannot encode value into query"));

        self.args.format_placeholder(&mut self.sql)
    }
}

pub trait ToQuery<'q, DB>: std::fmt::Display
where
    DB: Database,
{
    fn to_query(&'q self) -> Result<(String, DB::Arguments<'q>), std::fmt::Error> {
        let mut ctx = ToQueryContext::<DB>::default();
        self.write(&mut ctx)?;
        Ok((ctx.sql, ctx.args))
    }

    fn write(&'q self, ctx: &mut ToQueryContext<'q, DB>) -> std::fmt::Result;
}

pub use bind::bind;
pub use boolean_factor::not;
pub use boolean_term::and;
pub use boolean_test::{is_not_truth_value, is_truth_value};
pub use character_string_literal::char_str_lit;
pub use comparison_predicate::{eq, gt, gte, lt, lte, neq};
pub use error::Error;
pub use identifier::id;
pub use insert::insert;
pub use numeric_value_expression::{add, sub};
pub use search_condition::or;
pub use select::select;
pub use signed_numeric_literal::signed_numeric_lit;
pub use sql_builder_macros::{bind, columns, id, lit, row_value, select_columns};
pub use term::{div, mult};
pub use truth_value::{False, True, Unknown};
pub use unsigned_numeric_literal::unsigned_numeric_lit;

sql_builder_macros::check_symbol_loops!();

/// Common operations possible with the SQL symbols.
pub trait Symbol: Sized {
    /// Build an SQL query that can be executed by sqlx.
    fn build_sqlx<'q, DB>(&'q self) -> (String, DB::Arguments<'q>)
    where
        DB: Database,
        Self: ToQuery<'q, DB>,
    {
        self.to_query().unwrap()
    }

    /// Transform the current symbol if the predicate is true.
    fn transform_if<T>(
        self,
        predicate: bool,
        transform: impl FnOnce(Self) -> T,
    ) -> either::Either<Self, T> {
        if !predicate {
            either::Either::Left(self)
        } else {
            either::Either::Right(transform(self))
        }
    }
}

pub mod prelude {
    pub use crate::helpers::*;
    pub use crate::Symbol;
    pub use crate::ToQuery;
}

pub mod helpers {
    use crate::{
        boolean_primary::NestedSearchCondition, boolean_term::And,
        column_name_list::ColumnNameLink, contextually_typed_row_value_constructor::RowValue,
        contextually_typed_row_value_constructor_element_list::RowElementLink,
        contextually_typed_row_value_expression_list::ContextuallyTypedRowExpressionLink,
        cross_join::CrossJoin, derived_column::AliasedColumn, grammar as G,
        identifier_chain::IdentifierLink, join_type::Inner, qualified_join::QualifiedJoinFragment,
        search_condition::Or, select::Select, select_sublist::SelectLink,
        table_expression::TableExpr, table_reference_list::TableReferenceLink, where_clause::Where,
    };

    pub type QuerySpecificationWithTransformedWhere<Qs, SearchCond> = Select<
        <Qs as QuerySpecification>::SelectList,
        TableExpr<
            <<Qs as QuerySpecification>::TableExpression as TableExpression>::FromClause,
            Where<SearchCond>,
            <<Qs as QuerySpecification>::TableExpression as TableExpression>::GroupByClause,
            <<Qs as QuerySpecification>::TableExpression as TableExpression>::HavingClause,
        >,
    >;

    pub trait QuerySpecification: Sized {
        type SelectList: G::SelectList;
        type TableExpression: G::TableExpression;

        /// Unwrap the query specification
        fn unwrap(self) -> Select<Self::SelectList, Self::TableExpression>;

        /// Removed duplicated rows
        fn distinct(self) -> Select<Self::SelectList, Self::TableExpression> {
            let Select {
                quantifier: _,
                select_list,
                table_expression,
            } = self.unwrap();
            Select {
                quantifier: Some(crate::select::SetQuantifier::Distinct),
                select_list,
                table_expression,
            }
        }

        /// All rows are selected
        fn all(self) -> Select<Self::SelectList, Self::TableExpression> {
            let Select {
                quantifier: _,
                select_list,
                table_expression,
            } = self.unwrap();
            Select {
                quantifier: Some(crate::select::SetQuantifier::All),
                select_list,
                table_expression,
            }
        }

        /// Set the condition to filter the rows.
        fn r#where<SearchCond>(
            self,
            cond: SearchCond,
        ) -> QuerySpecificationWithTransformedWhere<Self, SearchCond>
        where
            Self: G::QuerySpecification,
            SearchCond: G::SearchCondition,
        {
            self.transform_table_expression(|expr| expr.r#where(cond))
        }

        /// Transform the table expression
        fn transform_table_expression<NewTableExpr>(
            self,
            transform: impl FnOnce(Self::TableExpression) -> NewTableExpr,
        ) -> Select<<Self as QuerySpecification>::SelectList, NewTableExpr>
        where
            NewTableExpr: G::TableExpression,
        {
            let Select {
                quantifier,
                select_list,
                table_expression,
            } = self.unwrap();

            Select {
                quantifier,
                select_list,
                table_expression: transform(table_expression),
            }
        }
    }

    pub trait FromClause {
        /// Add a table reference to the current from clause.
        fn add_table_reference(self, table_refs: impl G::TableReference) -> impl G::FromClause;
    }

    pub trait TableExpression: Sized {
        type FromClause: G::FromClause;
        type WhereClause: G::WhereClause;
        type GroupByClause: G::GroupByClause;
        type HavingClause: G::HavingClause;

        fn unwrap(
            self,
        ) -> TableExpr<Self::FromClause, Self::WhereClause, Self::GroupByClause, Self::HavingClause>;

        /// Transform the from clause
        fn transform_from<NewFromClause: G::FromClause>(
            self,
            transform: impl FnOnce(Self::FromClause) -> NewFromClause,
        ) -> TableExpr<NewFromClause, Self::WhereClause, Self::GroupByClause, Self::HavingClause>
        {
            let TableExpr {
                from_clause,
                where_clause,
                group_by,
                having,
            } = self.unwrap();

            TableExpr {
                from_clause: transform(from_clause),
                where_clause,
                group_by,
                having,
            }
        }

        /// Transform the where clause
        fn transform_where<NewWhereClause: G::WhereClause>(
            self,
            transform: impl FnOnce(Self::WhereClause) -> NewWhereClause,
        ) -> TableExpr<Self::FromClause, NewWhereClause, Self::GroupByClause, Self::HavingClause>
        {
            let TableExpr {
                from_clause,
                where_clause,
                group_by,
                having,
            } = self.unwrap();

            TableExpr {
                from_clause,
                where_clause: transform(where_clause),
                group_by,
                having,
            }
        }

        fn r#where<Cond>(
            self,
            cond: Cond,
        ) -> TableExpr<
            <Self as TableExpression>::FromClause,
            Where<Cond>,
            <Self as TableExpression>::GroupByClause,
            <Self as TableExpression>::HavingClause,
        >
        where
            Self: G::TableExpression,
            Cond: G::SearchCondition,
        {
            self.transform_where(|_| Where::new(cond))
        }
    }

    pub trait SelectSublist {
        fn add_selection<Element>(self, element: Element) -> SelectLink<Self, Element>
        where
            Self: G::SelectSublist,
            Element: G::SelectSublistElement,
        {
            SelectLink::new(self, element)
        }
    }

    pub trait ValueExpression {
        /// Alias the column.
        fn alias_column<Alias>(self, alias: Alias) -> AliasedColumn<Self, Alias>
        where
            Self: G::ValueExpression,
            Alias: G::ColumnName,
        {
            AliasedColumn::new(self, alias)
        }
    }

    pub trait Insert {
        type Target: G::InsertionTarget;
        type ColumnsAndSources: G::InsertColumnsAndSources;

        fn transform_target<NewTarget: G::InsertionTarget>(
            self,
            transform: impl FnOnce(Self::Target) -> NewTarget,
        ) -> impl G::Insert;

        fn transform_columns_and_sources<NewColumnsAndSources: G::InsertColumnsAndSources>(
            self,
            transform: impl FnOnce(Self::ColumnsAndSources) -> NewColumnsAndSources,
        ) -> impl G::Insert;
    }

    pub trait TableReferenceList {
        fn add_table_reference(
            self,
            table_ref: impl G::TableReference,
        ) -> impl G::TableReferenceList
        where
            Self: G::TableReferenceList,
        {
            TableReferenceLink::new(self, table_ref)
        }
    }
    pub trait TableReference: Sized {
        /// Cross join the table with another one.
        fn cross_join<Dest>(self, dest: Dest) -> CrossJoin<Self, Dest>
        where
            Self: G::TableReference,
            Dest: G::TablePrimary,
        {
            CrossJoin::new(self, dest)
        }

        /// Inner join the table with another one.
        fn inner_join<Dest>(self, dest: Dest) -> QualifiedJoinFragment<Self, Dest, Inner>
        where
            Self: G::TableReference,
            Dest: G::TablePrimary,
        {
            QualifiedJoinFragment::new(self, dest, Inner)
        }
    }

    pub trait ColumnNameList {
        fn add_column<Name>(self, column_name: Name) -> ColumnNameLink<Self, Name>
        where
            Self: G::ColumnNameList,
            Name: G::ColumnName,
        {
            ColumnNameLink::new(self, column_name)
        }
    }

    pub trait IdentifierChain {
        fn add_identifier<Id>(self, id: Id) -> IdentifierLink<Self, Id>
        where
            Self: G::IdentifierChain,
            Id: G::Identifier,
        {
            IdentifierLink::new(self, id)
        }
    }
    pub trait SearchCondition
    where
        Self: Sized,
    {
        fn nest(self) -> impl G::BooleanPrimary
        where
            Self: G::SearchCondition,
        {
            NestedSearchCondition::new(self)
        }

        fn or<Term>(self, rhs: Term) -> Or<Self, Term>
        where
            Self: G::SearchCondition,
            Term: G::BooleanTerm,
        {
            crate::or(self, rhs)
        }
    }

    pub trait BooleanTerm
    where
        Self: Sized + G::BooleanTerm,
    {
        fn and<Factor>(self, rhs: Factor) -> And<Self, Factor>
        where
            Factor: G::BooleanFactor,
        {
            crate::and(self, rhs)
        }
    }

    pub trait ContextuallyTypedRowValueExpressionList: Sized {
        /// Add a new row value in the list
        fn add_row_value<Value>(
            self,
            value: Value,
        ) -> ContextuallyTypedRowExpressionLink<Self, Value>
        where
            Self: G::ContextuallyTypedRowValueExpressionList,
            Value: G::ContextuallyTypedRowValueExpression,
        {
            ContextuallyTypedRowExpressionLink::new(self, value)
        }
    }

    pub trait ContextuallyTypedRowValueConstructorElementList: Sized {
        /// Transform a list of row value elements into a row value.
        fn into_row_value(self) -> RowValue<Self>
        where
            Self: G::ContextuallyTypedRowValueConstructorElementList,
        {
            RowValue::new(self)
        }

        /// Add a new row element in the list.
        fn add_row_element<Element>(self, element: Element) -> RowElementLink<Self, Element>
        where
            Self: G::ContextuallyTypedRowValueConstructorElementList,
            Element: G::ContextuallyTypedRowValueConstructorElement,
        {
            RowElementLink::new(self, element)
        }
    }
}

pub mod grammar {
    use sql_builder_macros::create_symbol_traits;
    create_symbol_traits! {}
}

/*
pub mod grammar {
    use crate::{
        boolean_factor::Not,
        boolean_primary::NestedSearchCondition,
        boolean_test::{IsNotTruthValue, IsTruthValue},
        contextually_typed_row_value_expression_list::ContextuallyTypedRowExpressionLink,
        either::Either,
        select_list::SelectLink,
        table_reference_list::TableRefList,
        ToQuery,
    };

    pub trait Select: Sized + ToQuery {
        type TableExpr: TableExpression;

        /// SELECT DISTINCT [...]
        fn distinct(self) -> impl Select;

        /// SELECT ALL [...]
        fn all(self) -> impl Select;

        /// See [self::TableExpression::append_from]
        fn append_from(self, table_refs: impl TableReferenceList) -> impl Select {
            self.transform_table_expression(|table_expr| table_expr.append_from(table_refs))
        }

        /// See [self::TableExpression::if_transform_from]
        fn if_transform_from<NewFromClause>(
            self,
            predicate: bool,
            then: impl FnOnce(<Self::TableExpr as TableExpression>::FromClause) -> NewFromClause,
        ) -> impl Select
        where
            NewFromClause: FromClause,
        {
            self.transform_table_expression(move |table_expr| {
                table_expr.if_transform_from(predicate, then)
            })
        }

        /// Transform the underlying table expression.
        fn transform_table_expression<NewTableExpr>(
            self,
            f: impl FnOnce(Self::TableExpr) -> NewTableExpr,
        ) -> impl Select
        where
            NewTableExpr: TableExpression;
    }

    /// A table expression
    ///
    /// # Grammar rule
    /// <table expression> ::= <from clause>
    /// [ <where clause> ]
    /// [ <group by clause> ]
    /// [ <having clause> ]
    pub trait TableExpression: ToQuery + Sized {
        type FromClause: FromClause;
        type WhereClause: WhereClause;

        /// Transform the from clause, and returns a new table expression
        fn transform_from<NewFromClause: FromClause>(
            self,
            transform: impl FnOnce(Self::FromClause) -> NewFromClause,
        ) -> impl TableExpression;

        /// Transform the where clause, and returns a new table expression
        fn transform_where<NewWhereClause: WhereClause>(
            self,
            transform: impl FnOnce(Self::WhereClause) -> NewWhereClause,
        ) -> impl TableExpression;

        /// Append table references to the current from clause.
        fn append_from(self, table_refs: impl TableReferenceList) -> impl TableExpression {
            self.transform_from(|from_clause| from_clause.add_table_references(table_refs))
        }

        /// Transform the from clause if the predicate is true.
        fn if_transform_from<NewFromClause>(
            self,
            predicate: bool,
            then: impl FnOnce(Self::FromClause) -> NewFromClause,
        ) -> impl TableExpression
        where
            NewFromClause: FromClause,
        {
            self.transform_from(move |from_clause| Either::if_else(predicate, from_clause, then))
        }

        /// Replace the current WHERE clause with another one.
        fn r#where(self, where_clause: impl SearchCondition) -> impl TableExpression {
            self.transform_where(|_| crate::Where::new(where_clause))
        }
    }

    /// A list of selected values.
    ///
    /// # Grammar rule
    /// <select list> ::= <asterisk> | (<derived column> | <qualifier> <period> <asterisk>)
    pub trait SelectList: Sized + ToQuery {
        const IS_IMPL: bool;

        /// Chain a new select expression
        fn append<T: SelectList>(self, next: T) -> SelectLink<Self, T> {
            SelectLink(self, next)
        }
    }

    /// A derived column.
    ///
    /// # Grammar rule
    /// ```ebnf
    /// <derived column> ::= <value expression> [ <as clause> ]
    /// ```
    ///
    /// Super: [self::SelectList]
    /// Children: [self::ValueExpression]
    pub trait DerivedColumn: SelectList + Sized {
        fn alias<ColName: ColumnName>(
            self,
            alias: ColName,
        ) -> crate::derived_column::DerivedColumn<Self, ColName>
        where
            Self: ValueExpression,
        {
            crate::derived_column::DerivedColumn {
                value_expression: self,
                alias,
            }
        }
    }

    /// A where clause
    ///
    /// # SQL
    /// ```sql
    /// WHERE <search condition>
    /// ```
    pub trait WhereClause: ToQuery {
        const IS_IMPL: bool;
    }

    /// A from clause
    ///
    /// Super: [self::TableExpression]
    ///
    /// # SQL
    /// ```sql
    /// FROM <table references>
    /// ```
    pub trait FromClause: TableExpression + ToQuery {
        /// Append table references to the from clause
        ///
        /// See [self::TableReferenceList]
        fn add_table_references(self, tab_refs: impl TableReferenceList) -> impl FromClause;
    }

    /// An having clause
    pub trait HavingClause: ToQuery {
        const IS_IMPL: bool;
    }

    /// A group by clause
    pub trait GroupByClause: ToQuery {
        const IS_IMPL: bool;
    }

    /// And order by clause
    pub trait OrderByClause: ToQuery {
        const IS_IMPL: bool;
    }

    /// Insert statement
    pub trait Insert: ToQuery + Sized {
        type Target: InsertionTarget;
        type ColumnsAndSources: InsertColumnsAndSources;

        /// Transform the target of the insert command.
        fn transform_target<NewTarget: InsertionTarget>(
            self,
            transform: impl FnOnce(Self::Target) -> NewTarget,
        ) -> impl Insert;

        /// Transform the inserted columns and sources.
        fn transform_columns_and_sources<NewColumnsAndSources: InsertColumnsAndSources>(
            self,
            transform: impl FnOnce(Self::ColumnsAndSources) -> NewColumnsAndSources,
        ) -> impl Insert;

        /// Add an insert column in the query.
        ///
        /// This only works if the inserted values are based on a constructor (see [self::FromConstructor]).
        /// See [self::InsertColumnList]
        fn add_insert_column(self, column_name: impl ColumnName) -> impl Insert
        where
            Self::ColumnsAndSources: FromConstructor,
        {
            self.transform_columns_and_sources(|from_constructor| {
                from_constructor.add_insert_column(column_name)
            })
        }
    }

    /// The target of the insertion.
    pub trait InsertionTarget: ToQuery {}

    /// What to insert, there are three ways : from a subquery, from a constructor, from default
    /// values.
    /// <insert columns and source> ::=  <from subquery> | <from constructor> | <from default>
    pub trait InsertColumnsAndSources: ToQuery + Sized {}

    /// Insert the results of a sub query.
    ///
    /// Super: [self::InsertColumnsAndSources]
    pub trait FromSubQuery: InsertColumnsAndSources + ToQuery {}

    /// Insert from constructed values.
    /// <from constructor> ::= [ <left paren> <insert column list> <right paren> ] [ <override clause> ] <contextually typed table value constructor>
    /// Super: [self::InsertColumnsAndSources]
    pub trait FromConstructor: InsertColumnsAndSources + ToQuery + Sized {
        type ColumnList: InsertColumnList;
        type TableValue: ContextuallyTypedTableValueConstructor;

        /// Transform the list of insert columns.
        fn transform_insert_column_list<NewColumnList: InsertColumnList>(
            self,
            transform: impl FnOnce(Self::ColumnList) -> NewColumnList,
        ) -> impl FromConstructor;

        /// See [self::InsertColumnList::add_insert_column]
        fn add_insert_column(self, column_name: impl ColumnName) -> impl FromConstructor {
            self.transform_insert_column_list(|list| list.add_insert_column(column_name))
        }
    }

    /// Insert default values.
    ///
    /// Super: [self::InsertColumnsAndSources]
    pub trait FromDefaults: InsertColumnsAndSources + ToQuery {}

    /// A list of columns which values are inserted.
    ///
    /// Children: [self::ColumnName]
    pub trait InsertColumnList: ToQuery {
        /// Add a new column name in the list.
        fn add_insert_column(self, column_name: impl ColumnName) -> impl InsertColumnList;
    }

    /// A constructor for a table value for insertion.
    ///
    /// <contextually typed table value constructor> ::= VALUES <contextually typed row value expression list>
    /// Super: [self::FromConstructor]
    pub trait ContextuallyTypedTableValueConstructor: FromConstructor + ToQuery {}

    /// A list of rows values.
    ///
    /// <contextually typed row value expression list> ::=
    /// <contextually typed row value expression>
    /// [ { <comma> <contextually typed row value expression> }... ]
    pub trait ContextuallyTypedRowValueExpressionList: Sized + ToQuery {
        fn add_row_value_expression(
            self,
            expr: impl ContextuallyTypedRowValueExpression,
        ) -> impl ContextuallyTypedRowValueExpressionList {
            ContextuallyTypedRowExpressionLink(self, expr)
        }
    }

    /// An expression to represents a row's values.
    ///
    /// It can be either expressed as :
    /// - a constructor ;
    /// - a value expression primary.
    ///
    /// <contextually typed row value expression> ::=
    /// <row value special case>
    /// | <contextually typed row value constructor>
    pub trait ContextuallyTypedRowValueExpression:
        ContextuallyTypedRowValueExpressionList + ToQuery
    {
    }

    /// A constructor for a row's values.
    ///
    /// <contextually typed row value constructor> ::=
    /// <contextually typed row value constructor element>
    /// | [ ROW ] <left paren> <contextually typed row value constructor element list> <right paren>
    pub trait ContextuallyTypedRowValueConstructor:
        ContextuallyTypedRowValueExpression + ToQuery
    {
    }

    /// A list of element to construct a row's values.
    ///
    /// <contextually typed row value constructor element list> ::=
    /// <contextually typed row value constructor element>
    /// [ { <comma> <contextually typed row value constructor element> }... ]
    pub trait ContextuallyTypedRowValueConstructorElementList:
        ContextuallyTypedRowValueConstructor + ToQuery
    {
    }

    /// An element of a row's values constructor.
    ///
    /// <contextually typed row value constructor element>  ::=
    /// <value expression>
    /// | <contextually typed value specification>
    pub trait ContextuallyTypedRowValueConstructorElement:
        ContextuallyTypedRowValueConstructor + ToQuery
    {
    }

    /// The specification of a value.
    ///
    /// <contextually typed value specification> ::=
    /// <implicitly typed value specification>
    /// | <default specification>
    pub trait ContextuallyTypedValueSpecification:
        ContextuallyTypedRowValueConstructorElement + ToQuery
    {
    }

    /// TODO!
    pub trait RowValueSpecialCase: ToQuery + ContextuallyTypedRowValueExpression {}

    /// A schema name
    /// <schema name> ::= [ <catalog name> <period> ] <unqualified schema name>
    pub trait SchemaName: ToQuery {}

    /// Unqualified schema name (without catalog name)
    pub trait UnqualifiedSchemaName: SchemaName + ToQuery {}

    /// <qualifier> ::= <table name> | <correlation name>
    ///
    /// Super:
    /// - [self::ColumnReference]
    /// - [self::SelectList]
    /// Children [self::TableName]
    pub trait Qualifier: ToQuery {}

    /// A qualified name <schema_name>. ? <identifier>
    ///
    /// Super: [self::TableReference]
    /// Children : [self::QualifiedIdentifier]
    pub trait QualifiedName: ToQuery + TableReference {}

    /// <qualified identifier> ::= <identifier>
    /// Super: [self::QualifiedName]
    /// Children: [self::Identifier]
    pub trait QualifiedIdentifier: QualifiedName + ToQuery {}

    /// <table reference list> ::= <table reference>
    /// | <table reference list> <comma> <table reference>
    pub trait TableReferenceList: ToQuery + Sized {
        fn chain<Rhs: TableReferenceList>(self, rhs: Rhs) -> TableRefList<Self, Rhs> {
            TableRefList { lhs: self, rhs }
        }
    }

    /// <table reference>    ::=
    /// <table name> [ <correlation specification> ]
    /// | <derived table> <correlation specification>
    /// | <joined table>
    ///
    /// Children [self::TableName]
    /// Super: [self::TableReferenceList]
    pub trait TableReference: TableReferenceList + ToQuery {}

    /// A table name
    /// <table name> ::= <qualified name> | <qualified local table name>
    ///
    /// Super:
    /// - [self::TableReference]
    /// - [self::InsertionTarget]
    /// Children [self::QualifiedName]
    pub trait TableName: ToQuery + TableReference + InsertionTarget {}

    /// <column reference> ::= [ <qualifier> <period> ] <column name>
    ///
    /// See [crate::column_reference::QualifiedColumnName]
    ///
    /// Super: [self::ValueExpressionPrimary]
    pub trait ColumnReference: ToQuery + ValueExpressionPrimary {}

    /// A comma separated list of column names.
    ///
    /// Super: [self::InsertColumnList]
    pub trait ColumnNameList: ToQuery + InsertColumnList {}

    /// A column name
    /// <column name> ::= <identifier>
    ///
    /// Super: [self::ColumnReference]
    /// Children: [self::Identifier]
    pub trait ColumnName: ColumnReference + ToQuery {}

    /// An identifier
    ///
    /// Super:
    /// - [self::QualifiedIdentifier]
    /// - [self::ColumnName]
    pub trait Identifier: QualifiedIdentifier + ColumnName {}

    /// <value expression>    ::=
    ///<numeric value expression>
    /// | <string value expression>
    /// | <datetime value expression>
    /// | <interval value expression>
    ///
    /// Super:
    /// - [self::DerivedColumn],
    /// - [self::RowValueConstructorElement]
    /// Children:
    /// - [self::NumericValueExpression],
    /// - [self::StringValueExpression],
    /// - [self::DateTimeValueExpression],
    /// - [self::IntervalValueExpression]
    pub trait ValueExpression:
        RowValueConstructorElement
        + DerivedColumn
        + ContextuallyTypedRowValueConstructorElement
        + ToQuery
    {
    }

    /// TODO!
    pub trait StringValueExpression: ValueExpression {}

    /// TODO!
    pub trait DateTimeValueExpression: ValueExpression {}

    /// TODO!
    pub trait IntervalValueExpression: ValueExpression {}

    /// <numeric value expression> ::= <term>
    /// | <numeric value expression> <plus sign> <term>
    /// | <numeric value expression> <minus sign> <term>
    ///
    /// Super: [self::ValueExpression]
    /// Children: [self::Term]
    pub trait NumericValueExpression: ValueExpression {}

    /// <term> ::= <factor>
    /// | <term> <asterisk> <factor>
    /// | <term> <solidus> <factor>
    ///
    /// Super: [self::NumericValueExpression]
    /// Children : [self::Factor]
    pub trait Term: NumericValueExpression {}

    pub trait Factor: Term {}

    /// <numeric primary> ::= <value expression primary> | <numeric value function>
    ///
    /// Super: [self::Factor]
    pub trait NumericPrimary: Factor {}

    /// TODO!
    pub trait NumericValueFunction {}

    /// The primary expression of a value
    ///
    /// ```ebnf
    /// <value expression primary> ::= <value specification>
    /// | <column reference>
    /// | <set function specification>
    /// | <scalar subquery>
    /// | <case expression>
    /// | <left paren> <value expression> <right paren>
    /// | <cast specification>
    /// | <bound value>
    /// ```
    ///
    /// # Differences with ISO/IEC 9075:1992
    ///
    /// *unsigned value specification* is replaced by *value specification*, and
    /// *bound value* is injected to allow parameters bindings for sqlx
    ///
    /// Super: [self::NumericPrimary]
    ///
    pub trait ValueExpressionPrimary: NumericPrimary {}

    /// A specified value
    /// Super: [self::ValueExpressionPrimary]
    pub trait ValueSpecification: ValueExpressionPrimary + ToQuery {}

    /// A literal
    ///
    /// Super: [self::ValueSpecification]
    pub trait Literal: ValueSpecification + ToQuery {}

    /// <search condition> ::= <boolean term>
    /// | <search condition> OR <boolean term>
    ///
    /// Children : [self::BooleanTerm]
    pub trait SearchCondition: ToQuery + Sized {
        /// Allows the search condition to be nested within another search condition.
        fn nest(self) -> impl BooleanPrimary {
            NestedSearchCondition(self)
        }
    }

    /// A boolean term.
    ///
    /// ```ebnf
    /// <boolean term> ::= <boolean factor> | <boolean term> AND <boolean factor>
    /// ```
    ///
    /// Super: [self::SearchCondition]
    /// Children : [self::BooleanFactor]
    pub trait BooleanTerm: SearchCondition + ToQuery {}

    /// <boolean factor> ::= [ NOT ] <boolean test>
    ///
    /// Super: [self::BooleanTerm]
    /// Children: [self::BooleanTest]
    pub trait BooleanFactor: BooleanTerm + ToQuery {}

    /// <boolean test> ::= <boolean primary> [ IS [ NOT ] <truth value> ]
    ///
    /// See : [crate::boolean_test::IsTruthValue, crate::boolean_test::IsNotTruthValue]
    ///
    /// Super: [self::BooleanFactor]
    /// Children: [self::BooleanPrimary]
    pub trait BooleanTest: BooleanFactor + Sized + ToQuery {
        fn not(self) -> Not<Self> {
            crate::boolean_factor::Not(self)
        }
    }

    /// <boolean primary> ::= <predicate> | <left paren> <search condition> <right paren>
    ///
    /// Super: [self::BooleanTest]
    pub trait BooleanPrimary: BooleanTest + Sized + ToQuery {
        #[allow(clippy::wrong_self_convention)]
        fn is<TruthVal: TruthValue>(self, truth_value: TruthVal) -> IsTruthValue<Self, TruthVal> {
            IsTruthValue {
                lhs: self,
                rhs: truth_value,
            }
        }

        #[allow(clippy::wrong_self_convention)]
        fn is_not<TruthVal: TruthValue>(
            self,
            truth_value: TruthVal,
        ) -> IsNotTruthValue<Self, TruthVal> {
            IsNotTruthValue {
                lhs: self,
                rhs: truth_value,
            }
        }
    }

    /// A predicate
    ///
    /// # Grammar rule
    /// ```ebnf
    /// (predicate) ::= (comparison predicate)
    /// | (between predicate)
    /// | (in predicate)
    /// | (like predicate)
    /// | (null predicate)
    /// | (quantified comparison predicate)
    /// | (exists predicate)
    /// | (match predicate)
    /// | (overlaps predicate)
    ///```
    /// Super: [self::BooleanPrimary]
    ///
    /// Children:
    /// - [self::ComparisonPredicate],
    /// - [self::BetweenPredicate],
    /// - [self::InPredicate],
    /// - [self::LikePredicate],
    /// - [self::NullPredicate],
    /// - [self::QuantifiedComparisonPredicate],
    /// - [self::ExistsPredicate],
    /// - [self::MatchPredicate]
    /// - [self::OverlapsPredicate]
    pub trait Predicate: BooleanPrimary + ToQuery {}

    /// A predicate based on values comparison.
    ///
    /// # Grammar rule
    /// ```ebnf
    /// <comparison predicate> ::= <row value constructor> <comp op> <row value constructor>
    /// ```
    ///
    /// Super: [self::Predicate]
    pub trait ComparisonPredicate: Predicate + ToQuery {}

    /// A predicate based on ranged values.
    ///
    /// # Grammar rule
    /// ```ebnf
    /// <between predicate> ::= <row value constructor> [ NOT ] BETWEEN <row value constructor> AND <row value constructor>
    /// ```
    ///
    /// Super: [self::Predicate]
    pub trait BetweenPredicate: Predicate + ToQuery {}

    /// A predicate that returns true if a value is within in a set.
    ///
    /// # Grammar rule
    /// ```ebnf
    /// <in predicate> ::= <row value constructor> [ NOT ] IN <in predicate value>
    /// ```
    /// Super: [self::Predicate]
    pub trait InPredicate: Predicate + ToQuery {}

    /// <like predicate> ::= <match value> [ NOT ] LIKE <pattern> [ ESCAPE <escape character> ]
    ///
    /// Super: [self::Predicate]
    pub trait LikePredicate: Predicate + ToQuery {}

    /// <null predicate> ::= <row value constructor> IS [ NOT ] NULL
    ///
    /// Super: [self::Predicate]
    pub trait NullPredicate: Predicate + ToQuery {}

    /// <quantified comparison predicate> ::= <row value constructor> <comp op> <quantifier> <table subquery>
    ///
    /// Super: [self::Predicate]
    pub trait QuantifiedComparisonPredicate: Predicate + ToQuery {}

    /// <exists predicate> ::= EXISTS <table subquery>
    ///
    /// Super: [self::Predicate]
    pub trait ExistsPredicate: Predicate + ToQuery {}
    /// <match predicate> ::= <row value constructor> MATCH [ UNIQUE ] [ PARTIAL | FULL ] <table subquery>
    ///
    /// Super: [self::Predicate]
    pub trait MatchPredicate: Predicate + ToQuery {}

    /// <overlaps predicate> ::= <row value constructor 1> OVERLAPS <row value constructor 2>
    ///
    /// Super: [self::Predicate]
    pub trait OverlapsPredicate: Predicate + ToQuery {}

    /// <truth value> ::= TRUE | FALSE | UNKNOWN
    pub trait TruthValue: ToQuery {}

    pub trait RowValueConstructor: ToQuery {}

    /// A comma-separated list of [self::RowValueElement]
    ///
    /// Super:
    /// - [self::RowValueConstructor]
    pub trait RowValueConstructorList: RowValueConstructor + ToQuery {}

    ///
    ///
    /// Super:
    /// - [self::RowValueConstructor]
    /// - [self::RowValueConstructorList]
    /// Children:
    /// - [self::ValueExpression]
    pub trait RowValueConstructorElement:
        RowValueConstructor + RowValueConstructorList + ToQuery
    {
    }
}*/
