pub mod error;

pub mod group_by;
pub mod name;
pub mod select;
pub mod select_list;
pub mod table_expression;
pub mod table_reference_list;

pub mod bind;
pub mod identifier;
pub mod literal;
pub mod numeric_value_expression;
pub mod term;
pub mod where_clause;

pub mod derived_column;
pub mod from_clause;

pub mod boolean_factor;
pub mod boolean_primary;
pub mod boolean_term;
pub mod boolean_test;
pub mod comparison_predicate;
pub mod contextually_typed_row_value_constructor_element_list;
pub mod contextually_typed_row_value_expression_list;
pub mod either;
pub mod having;
pub mod insert;
pub mod schema_name;
pub mod search_condition;
pub mod unqualified_schema_name;

use std::io::Write;

#[derive(Default)]
pub struct ToQueryContext {}
pub trait ToQuery {
    fn to_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let mut bytes = Vec::<u8>::default();
        let mut ctx = ToQueryContext::default();
        self.write(&mut bytes, &mut ctx)?;
        let sql = String::from_utf8(bytes)?;
        Ok(sql)
    }

    fn write<W: Write>(
        &self,
        stream: &mut W,
        ctx: &mut ToQueryContext,
    ) -> Result<(), std::io::Error>;
}

impl ToQuery for () {
    fn write<W: Write>(
        &self,
        _stream: &mut W,
        _ctx: &mut ToQueryContext,
    ) -> Result<(), std::io::Error> {
        Ok(())
    }
}

pub use boolean_factor::not;
pub use boolean_term::and;
pub use boolean_test::{is_not_truth_value, is_truth_value};
pub use comparison_predicate::{eq, gt, gte, lt, lte, neq};
pub use identifier::id;
pub use literal::lit;
pub use numeric_value_expression::{add, sub};
pub use search_condition::or;
pub use select::select;
pub use term::{div, mult};
pub use where_clause::Where;

pub use sql_builder_macros::id;

#[derive(Default)]
/// Blank type for default symbol trait implementation.
pub struct Blank();

impl ToQuery for Blank {
    fn write<W: Write>(
        &self,
        _stream: &mut W,
        _ctx: &mut ToQueryContext,
    ) -> Result<(), std::io::Error> {
        Ok(())
    }
}

pub mod helpers {
    use crate::grammar as G;
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
