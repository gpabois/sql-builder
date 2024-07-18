pub mod error;

pub mod group_by;
pub mod name;
pub mod select;
pub mod select_list;

pub mod bind;
pub mod identifier;
pub mod term;
pub mod r#where;

pub mod derived_column;
pub mod from;

pub mod boolean_factor;
pub mod boolean_term;
pub mod boolean_test;
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

pub mod traits {
    use crate::{
        boolean_factor::Not,
        boolean_test::{IsNotTruthValue, IsTruthValue},
        ToQuery,
    };

    pub trait QueryExpression {}

    pub trait SelectStatement {}

    impl<T> QueryExpression for T where T: SelectStatement {}

    pub trait SelectList: Sized + ToQuery {
        const IS_IMPL: bool;

        /// Chain a new select expression
        fn chain<T: SelectList>(self, next: T) -> crate::select_list::SelectList<Self, T> {
            crate::select_list::SelectList(self, next)
        }
    }

    impl SelectList for () {
        const IS_IMPL: bool = false;
    }

    /// <derived column> ::= <value expression> [ <as clause> ]
    ///
    /// Inherits: [self::SelectList]
    /// Inherited by: [self::ValueExpression]
    pub trait DerivedColumn: SelectList + Sized {
        fn r#as<ColName: ColumnName>(
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

    pub trait WhereClause: ToQuery {
        const IS_IMPL: bool;
    }
    impl WhereClause for () {
        const IS_IMPL: bool = false;
    }

    pub trait FromClause: ToQuery {
        const IS_IMPL: bool;
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
    /// Inherits:
    /// - [self::ColumnReference]
    /// - [self::SelectList]
    /// Inherited by [self::TableName]
    pub trait Qualifier {}

    /// A qualified name <schema_name>. ? <identifier>
    ///
    /// Inherits: [self::TableReference]
    /// Inherited by : [self::QualifiedIdentifier]
    pub trait QualifiedName: ToQuery + TableReference {}

    /// <qualified identifier> ::= <identifier>
    /// Inherits: [self::QualifiedName]
    /// Inherited by: [self::Identifier]
    pub trait QualifiedIdentifier: QualifiedName + ToQuery {}

    /// <table reference>    ::=
    /// <table name> [ <correlation specification> ]
    /// | <derived table> <correlation specification>
    /// | <joined table>
    ///
    /// Inherited by [self::TableName]
    pub trait TableReference: ToQuery {}

    /// A table name
    /// <table name> ::= <qualified name> | <qualified local table name>
    ///
    /// Inherited by [self::QualifiedName]
    pub trait TableName: ToQuery + TableReference {}

    /// <column reference> ::= [ <qualifier> <period> ] <column name>
    ///
    /// See [crate::column_reference::QualifiedColumnName]
    ///
    /// Inherits: [self::ValueExpressionPrimary]
    pub trait ColumnReference: ToQuery + ValueExpressionPrimary {}

    /// A column name
    /// <column name> ::= <identifier>
    ///
    /// Inherited by: [self::Identifier]
    pub trait ColumnName: ToQuery {}

    /// An identifier
    ///
    /// Inherits:
    /// - [self::QualifiedIdentifier]
    /// - [self::ColumnName]
    pub trait Identifier: QualifiedIdentifier + ColumnName {}

    /// <value expression>    ::=
    ///<numeric value expression>
    /// | <string value expression>
    /// | <datetime value expression>
    /// | <interval value expression>
    ///
    /// Inherits: [self::DerivedColumn]
    /// Inherited by:
    /// - [self::NumericValueExpression],
    /// - [self::StringValueExpression],
    /// - [self::DateTimeValueExpression],
    /// - [self::IntervalValueExpression]
    pub trait ValueExpression: DerivedColumn {}

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
    /// Inherits: [self::ValueExpression]
    /// Inherited by: [self::Term]
    pub trait NumericValueExpression: ValueExpression {}

    /// <term> ::= <factor>
    /// | <term> <asterisk> <factor>
    /// | <term> <solidus> <factor>   
    ///
    /// Inherits: [self::NumericValueExpression]
    /// Inherited by : [self::Factor]
    pub trait Term: NumericValueExpression {}

    pub trait Factor: Term {}

    /// <numeric primary> ::= <value expression primary> | <numeric value function>
    ///
    /// Inherits: [self::Factor]
    pub trait NumericPrimary: Factor {}

    /// TODO!
    pub trait NumericValueFunction {}

    /// <value expression primary>    ::=
    /// <unsigned value specification>
    /// | <column reference>
    /// | <set function specification>
    /// | <scalar subquery>
    /// | <case expression>
    /// | <left paren> <value expression> <right paren>
    /// | <cast specification>
    /// | <bound value>
    ///
    /// Inherits: [self::NumericPrimary]
    pub trait ValueExpressionPrimary: NumericPrimary {}

    /// <search condition> ::= <boolean term>
    /// | <search condition> OR <boolean term>
    ///
    /// Inherited by : [self::BooleanTerm]
    pub trait SearchCondition: ToQuery {}

    /// <boolean term> ::= <boolean factor>
    /// | <boolean term> AND <boolean factor>
    ///
    /// Inherits: [self::SearchCondition]
    /// Inherited by : [self::BooleanFactor]
    pub trait BooleanTerm: SearchCondition + ToQuery {}

    /// <boolean factor> ::= [ NOT ] <boolean test>
    ///
    /// Inherits: [self::BooleanTerm]
    /// Inherited by: [self::BooleanTest]
    pub trait BooleanFactor: BooleanTerm + ToQuery {}

    /// <boolean test> ::= <boolean primary> [ IS [ NOT ] <truth value> ]
    ///
    /// See : [crate::boolean_test::IsTruthValue, crate::boolean_test::IsNotTruthValue]
    ///
    /// Inherits: [self::BooleanTerm]
    /// Inherited by: [self::BooleanPrimary]
    pub trait BooleanTest: BooleanTerm + Sized + ToQuery {
        fn not(self) -> Not<Self> {
            crate::boolean_factor::Not(self)
        }
    }

    /// <boolean primary> ::= <predicate> | <left paren> <search condition> <right paren>
    ///
    /// Inherits: [self::BooleanTest]
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

    /// <predicate> ::= <comparison predicate>
    /// | <between predicate>
    /// | <in predicate>
    /// | <like predicate>
    /// | <null predicate>
    /// | <quantified comparison predicate>
    /// | <exists predicate>
    /// | <match predicate>
    /// | <overlaps predicate>
    ///
    /// Inherits: [self::BooleanPrimary]
    /// Inherited by :
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

    /// <comparison predicate> ::= <row value constructor> <comp op> <row value constructor>
    ///
    /// Inherits: [self::Predicate]
    pub trait ComparisonPredicate: Predicate + ToQuery {}
    /// <between predicate> ::= <row value constructor> [ NOT ] BETWEEN <row value constructor> AND <row value constructor>
    ///
    /// Inherits: [self::Predicate]
    pub trait BetweenPredicate: Predicate + ToQuery {}
    /// <in predicate> ::= <row value constructor> [ NOT ] IN <in predicate value>
    ///
    /// Inherits: [self::Predicate]
    pub trait InPredicate: Predicate + ToQuery {}
    /// <like predicate> ::= <match value> [ NOT ] LIKE <pattern> [ ESCAPE <escape character> ]
    ///
    /// Inherits: [self::Predicate]
    pub trait LikePredicate: Predicate + ToQuery {}

    /// <null predicate> ::= <row value constructor> IS [ NOT ] NULL
    ///
    /// Inherits: [self::Predicate]
    pub trait NullPredicate: Predicate + ToQuery {}

    /// <quantified comparison predicate> ::= <row value constructor> <comp op> <quantifier> <table subquery>
    ///
    /// Inherits: [self::Predicate]
    pub trait QuantifiedComparisonPredicate: Predicate + ToQuery {}

    /// <exists predicate> ::= EXISTS <table subquery>
    ///
    /// Inherits: [self::Predicate]
    pub trait ExistsPredicate: Predicate + ToQuery {}
    /// <match predicate> ::= <row value constructor> MATCH [ UNIQUE ] [ PARTIAL | FULL ] <table subquery>
    ///
    /// Inherits: [self::Predicate]
    pub trait MatchPredicate: Predicate + ToQuery {}

    /// <overlaps predicate> ::= <row value constructor 1> OVERLAPS <row value constructor 2>
    ///
    /// Inherits: [self::Predicate]
    pub trait OverlapsPredicate: Predicate + ToQuery {}

    /// <truth value> ::= TRUE | FALSE | UNKNOWN
    pub trait TruthValue {}
}
