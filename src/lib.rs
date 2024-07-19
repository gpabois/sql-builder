pub mod error;

pub mod group_by;
pub mod name;
pub mod select;
pub mod select_list;
pub mod table_expression;
pub mod table_reference_list;

pub mod bind;
pub mod identifier;
pub mod term;
pub mod numeric_value_expression;
pub mod r#where;
pub mod literal;

pub mod derived_column;
pub mod from;

pub mod comparison_predicate;
pub mod boolean_factor;
pub mod boolean_term;
pub mod boolean_test;
pub mod boolean_primary;
pub mod search_condition;

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

pub use r#where::Where;
pub use select::select;
pub use search_condition::or;
pub use boolean_term::and;
pub use boolean_factor::not;
pub use boolean_test::{is_not_truth_value, is_truth_value};
pub use comparison_predicate::{eq, neq, lte, lt, gt, gte};
pub use numeric_value_expression::{add, sub};
pub use term::{mult, div};
pub use identifier::id;
pub use literal::lit;

pub mod grammar {
    use crate::{boolean_factor::Not, boolean_primary::NestedSearchCondition, boolean_test::{IsNotTruthValue, IsTruthValue}, from::From, select_list::SelectLink, table_expression::{self, TableExpr}, table_reference_list::TableRefList, r#where::Where, ToQuery};

    pub trait Select: Sized + ToQuery {
        /// SELECT DISTINCT [...]
        fn distinct(self) -> impl Select;
        
        /// SELECT ALL [...]
        fn all(self) -> impl Select;

        /// Append table references to the from clause
        /// 
        /// See [self::FromClause::add_table_references]
        fn and_from(self, table_refs: impl TableReferenceList) -> impl Select;
    }   

    /// A table expression
    /// 
    /// # Grammar rule
    /// <table expression> ::= <from clause>
    /// [ <where clause> ]
    /// [ <group by clause> ]
    /// [ <having clause> ] 
    pub trait TableExpression: ToQuery + Sized {      
        /// Add the table references to the current from clause. 
        fn and_from(self, table_refs: impl TableReferenceList) -> impl TableExpression;
        
        /// Replace the current WHERE clause with another one.
        fn r#where(self, where_clause: impl SearchCondition) -> impl TableExpression;
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

    impl SelectList for () {
        const IS_IMPL: bool = false;
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
    impl WhereClause for () {
        const IS_IMPL: bool = false;
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

    impl TableExpression for () {
        fn r#where(self, search_cond: impl SearchCondition) -> impl TableExpression {
            TableExpr {
                from_clause: (),
                where_clause: Where::new(search_cond),
                group_by: (),
                having: ()
            }
        }
        
        fn and_from(self, table_refs: impl TableReferenceList) -> impl TableExpression {
            TableExpr {
                from_clause: From::new(table_refs),
                where_clause: (),
                group_by: (),
                having: ()
            }
        }
    }

    impl FromClause for () {
        fn add_table_references(self, table_refs: impl TableReferenceList) -> impl FromClause 
        {
            From{ table_refs }
        }
    }
    
    pub trait HavingClause {}
    impl HavingClause for () {}

    pub trait GroupByClause {}
    impl GroupByClause for () {}

    pub trait OrderByClause {}
    impl OrderByClause for () {}

    pub trait LimitExpr {}
    impl LimitExpr for () {}

    /// (<catalog_name> .)? <schema_name> .
    pub trait SchemaName {}

    /// <qualifier> ::= <table name> | <correlation name>
    ///
    /// Super:
    /// - [self::ColumnReference]
    /// - [self::SelectList]
    /// Children [self::TableName]
    pub trait Qualifier {}

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
            TableRefList {
                lhs: self,
                rhs
            }
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
    /// Children [self::QualifiedName]
    pub trait TableName: ToQuery + TableReference {}

    /// <column reference> ::= [ <qualifier> <period> ] <column name>
    ///
    /// See [crate::column_reference::QualifiedColumnName]
    ///
    /// Super: [self::ValueExpressionPrimary]
    pub trait ColumnReference: ToQuery + ValueExpressionPrimary {}

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
    pub trait ValueExpression: RowValueConstructorElement + DerivedColumn + ToQuery {}

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
        fn is<TruthVal: TruthValue>(self, truth_value: TruthVal) -> IsTruthValue<Self, TruthVal> {
            IsTruthValue {
                lhs: self,
                rhs: truth_value,
            }
        }

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
    pub trait TruthValue : ToQuery {}

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
    pub trait RowValueConstructorElement: RowValueConstructor + RowValueConstructorList + ToQuery {}
}
