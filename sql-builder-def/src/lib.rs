use itertools::Itertools;
use phf::phf_map;

pub struct SymbolDef {
    flags: u8,
    pub deps: &'static [&'static str],
}

impl SymbolDef {
    #[inline]
    pub const fn new(deps: &'static [&'static str], flags: u8) -> Self {
        Self { flags, deps }
    }

    #[inline]
    pub fn with_either_impl(&self) -> bool {
        self.flags & WITH_EITHER_IMPL == WITH_EITHER_IMPL
    }

    #[inline]
    pub fn with_blank_impl(&self) -> bool {
        self.flags & WITH_BLANK_IMPL == WITH_BLANK_IMPL
    }

    #[inline]
    pub fn with_helpers(&self) -> bool {
        self.flags & WITH_HELPERS == WITH_HELPERS
    }

    #[inline]
    pub fn helpers_require_methods_implementations(&self) -> bool {
        self.flags & WITH_REQUIRED_HELPERS_METHOD == WITH_REQUIRED_HELPERS_METHOD
    }
}

pub const WITH_BLANK_IMPL: u8 = 0b1;
pub const WITH_EITHER_IMPL: u8 = 0b10;
/// The symbol has helper functions.
pub const WITH_HELPERS: u8 = 0b100;
/// Used when the helper trait requires methods implementation.
pub const WITH_REQUIRED_HELPERS_METHOD: u8 = 0b1000;

/// Defines the symbols in the SQL grammar.
pub static SYMBOL_MAP: phf::Map<&'static str, SymbolDef> = phf_map! {
    // *
    "Asterisk" => SymbolDef::new(&[], 0),

    // <query expression> ::= [ <with clause> ] <query expression body>
    "QueryExpression" => SymbolDef::new(&["QueryExpressionBody"], 0),

    // <query expression body> ::= <non-join query expression> | <joined table>
    "QueryExpressionBody" => SymbolDef::new(&[
        "NonJoinQueryExpression",
        "JoinedTable"
    ], 0),

    // # Joined table
    /*
        <joined table> ::=
            <cross join>
            | <qualified join>
            | <natural join>
            | <union join>
    */
    "JoinedTable" => SymbolDef::new(&[
        "CrossJoin",
        "QualifiedJoin",
        "NaturalJoin",
        "UnionJoin"
    ], 0),

    // <cross join> ::= <table reference> CROSS JOIN <table primary>
    "CrossJoin" => SymbolDef::new(&[], 0),

    // <qualified join> ::= <table reference> [ <join type> ] JOIN <table reference> <join specification>
    "QualifiedJoin" => SymbolDef::new(&[], 0),

    // <natural join> ::= <table reference> NATURAL [ <join type> ] JOIN <table primary>
    "NaturalJoin" => SymbolDef::new(&[], 0),

    // <union join> ::= <table reference> UNION JOIN <table primary>
    "UnionJoin" => SymbolDef::new(&[], 0),

    // <join specification>    ::=   <join condition> | <named columns join>
    "JoinSpecification" => SymbolDef::new(&["JoinCondition", "NamedColumnsJoin"], 0),

    // <join condition> ::= ON <search condition>
    "JoinCondition" => SymbolDef::new(&[], 0),

    // <named columns join> ::= USING <left paren> <join column list> <right paren>
    "NamedColumnsJoin" => SymbolDef::new(&[], 0),

    // <join type> ::=  INNER | <outer join type> [ OUTER ]
    "JoinType" => SymbolDef::new(&[], WITH_BLANK_IMPL),

    // <outer join type> ::= LEFT | RIGHT | FULL
    "OuterJoinType" => SymbolDef::new(&[], 0),

    // <join column list> ::= <column name list>
    "JoinColumnList" => SymbolDef::new(&["ColumnNameList"], 0),

    /*
    * <non-join query expression>    ::=
           <non-join query term>
         | <query expression body> UNION [ ALL | DISTINCT ] [ <corresponding spec> ] <query term>
         | <query expression body> EXCEPT [ ALL | DISTINCT ] [ <corresponding spec> ] <query term>
    * */
    "NonJoinQueryExpression" => SymbolDef::new(&["NonJoinQueryTerm"], 0),

    /*
    * <non-join query term>    ::=
         <non-join query primary>
        | <query term> INTERSECT [ ALL | DISTINCT ] [ <corresponding spec> ] <query primary>
    * */
    "NonJoinQueryTerm" => SymbolDef::new(&["NonJoinQueryPrimary"], 0),

    /*
        <non-join query primary> ::=
            <simple table>
            | <left paren> <non-join query expression> <right paren>
    */
    "NonJoinQueryPrimary" => SymbolDef::new(&["SimpleTable"], 0),

    /*
        <simple table>    ::=
            <query specification>
            | <table value constructor>
            | <explicit table>

    */
    "SimpleTable" => SymbolDef::new(&["QuerySpecification"], 0),

    /*
        <query specification> ::=
            SELECT [ <set quantifier> ]
                <select list>
                <table expression>
    */
    "QuerySpecification" => SymbolDef::new(&[],
        WITH_HELPERS | WITH_REQUIRED_HELPERS_METHOD
    ),
    /*
        <table expression>    ::=
            <from clause>
            [ <where clause> ]
            [ <group by clause> ]
            [ <having clause> ]
            [ <window clause> ]
    */
    "TableExpression" => SymbolDef::new(&[
        "FromClause"
    ], WITH_HELPERS | WITH_EITHER_IMPL | WITH_REQUIRED_HELPERS_METHOD),

    /*
        <from clause> ::= FROM <table reference list>
    */
    "FromClause" => SymbolDef::new(&[], WITH_HELPERS | WITH_BLANK_IMPL | WITH_EITHER_IMPL | WITH_REQUIRED_HELPERS_METHOD),

    // # WHERE CLAUSE

    //<where clause> ::= WHERE <search condition>
    "WhereClause" => SymbolDef::new(&[], WITH_BLANK_IMPL | WITH_EITHER_IMPL),
    /*
        <group by clause> ::= GROUP BY [ <set quantifier> ] <grouping element list>
    */
    "GroupByClause" => SymbolDef::new(&[], WITH_BLANK_IMPL),

    /*
        <having clause> ::= HAVING <search condition>
    */
    "HavingClause" => SymbolDef::new(&[], WITH_BLANK_IMPL),

    /*
        <select list> ::=
            <asterisk>
            | <select sublist>
    */
    "SelectList" => SymbolDef::new(&[
        "SelectSublist",
        "Asterisk"
    ], WITH_BLANK_IMPL),

    /*
        <select sublist> ::=
            | <select sublist element>
            | <select sublist> [ { <comma> <select sublist element> }... ]

        Difference with the ISO/IEC 9075-2:2003 :
        Recursive <select sublist> chaining.
    */
    "SelectSublist" => SymbolDef::new(&[
        "SelectSublistElement",
    ], WITH_BLANK_IMPL | WITH_HELPERS),

    /*
        <select sublist element> ::=
            <derived column>
            | <qualified asterisk>
     */
    "SelectSublistElement" => SymbolDef::new(&[
        "DerivedColumn",
        "QualifiedAsterisk"
    ], 0),

    /*
        <qualified asterisk>    ::=
            <asterisked identifier chain> <period> <asterisk>
            | <all fields reference>
    */
    "QualifiedAsterisk" => SymbolDef::new(&[
        "AllFieldsReference"
    ], 0),

    /*
        <all fields reference> ::=
        <value expression primary> <period> <asterisk>
        [ AS <left paren> <all fields column name list> <right paren> ]
    */
    "AllFieldsReference" => SymbolDef::new(&[], 0),

    /*
        <derived column> ::= <value expression> [ <as clause> ]
    */
    "DerivedColumn" => SymbolDef::new(&[
        "ValueExpression"
    ], 0),

    /*
        <value expression>    ::=
            <common value expression>
            | <boolean value expression>
            | <row value expression>
    */
    "ValueExpression" => SymbolDef::new(&[
        "CommonValueExpression",
        "BooleanValueExpression",
        "RowValueExpression"
    ], WITH_HELPERS),

    /*
        <common value expression>    ::=
            <numeric value expression>
            | <string value expression>
            | <datetime value expression>
            | <interval value expression>
            | <user-defined type value expression>
            | <reference value expression>
            | <collection value expression>
    */
    "CommonValueExpression" => SymbolDef::new(&[
        "NumericValueExpression",
        "StringValueExpression",
        "DatetimeValueExpression",
        "IntervalValueExpression",
        "UserDefinedTypeValueExpression",
        "ReferenceValueExpression",
        "CollectionValueExpression"
    ], 0),

    /*
        <numeric value expression>    ::=
            <term>
            | <numeric value expression> <plus sign> <term>
            | <numeric value expression> <minus sign> <term>
    */
    "NumericValueExpression" => SymbolDef::new(&["Term"], 0),
    "StringValueExpression" => SymbolDef::new(&[], 0),
    "DatetimeValueExpression" => SymbolDef::new(&[], 0),
    "IntervalValueExpression" => SymbolDef::new(&[], 0),
    "UserDefinedTypeValueExpression" => SymbolDef::new(&[], 0),
    "ReferenceValueExpression" => SymbolDef::new(&[], 0),
    "CollectionValueExpression" => SymbolDef::new(&[], 0),

    /*
        <term> ::=
            <factor>
            | <term> <asterisk> <factor>
            | <term> <solidus> <factor>
    */
    "Term" => SymbolDef::new(&["Factor"], 0),

    /*
        <factor> ::= [ <sign> ] <numeric primary>
    */
    "Factor" => SymbolDef::new(&["NumericPrimary"], 0),

    /*
        <numeric primary> ::=
            <value expression primary>
            | <numeric value function>
    */
    "NumericPrimary" => SymbolDef::new(&[
        "ValueExpressionPrimary",
        "NumericValueFunction"
    ], 0),

    /*
        <numeric value function>    ::=
            <position expression>
            | <extract expression>
            | <length expression>
            | <cardinality expression>
            | <absolute value expression>
            | <modulus expression>
            | <natural logarithm>
            | <exponential function>
            | <power function>
            | <square root>
            | <floor function>
            | <ceiling function>
            | <width bucket function>
    */
    "NumericValueFunction" => SymbolDef::new(&[
        "PositionExpression",
        "ExtractExpression",
        "LengthExpression",
        "CardinalityExpression",
        "AbsoluteValueExpression",
        "ModulusExpression",
        "NaturalLogarithm",
        "ExponentialFunction",
        "PowerFunction",
        "SquareRoot",
        "FloorFunction",
        "CeilingFunction",
        "WidthBucketFunction"
    ], 0),

    /*
        <width bucket function> ::= WIDTH_BUCKET (
            <width bucket operand>,
            <width bucket bound 1>,
            <width bucket bound 2>,
            <width bucket count>,
        )
    */
    "WidthBucketFunction" => SymbolDef::new(&[], 0),
    /*
        <width bucket operand> ::= <numeric value expression>
    */
    "WidthBucketOperand" => SymbolDef::new(&[], 0),
    /*
        <width bucket bound 1> ::= <numeric value expression>
    */
    "WidthBucketBound1" => SymbolDef::new(&[], 0),
    /*
        <width bucket bound 2> ::= <numeric value expression>
    */
    "WidthBucketBound2" => SymbolDef::new(&[], 0),
    /*
        <width bucket count> ::= <numeric value expression>
    */
    "WidthBucketCount" => SymbolDef::new(&[], 0),
    /*
        <ceiling function> ::= FLOOR (
            <numeric value expression>
        )
    */
    "CeilingFunction" => SymbolDef::new(&[], 0),
    /*
        <floor function> ::= FLOOR (
            <numeric value expression>
        )
    */
    "FloorFunction" => SymbolDef::new(&[], 0),
    /*
        <square root> ::= SQRT (
            <numeric value expression>
        )
    */
    "SquareRoot" => SymbolDef::new(&[], 0),
    /*
        <power function> ::= POWER (
            <numeric value expression base>,
            <numeric value expression exponent>
        )
    */
    "PowerFunction" => SymbolDef::new(&[], 0),
    /*
        <exponential function> ::= EXP (
            <numeric value expression>
        )
    */
    "ExponentialFunction" => SymbolDef::new(&[], 0),
    /*
        <natural logarithm> ::= LN (
            <numeric value expression>
        )
    */
    "NaturalLogarithm" => SymbolDef::new(&[], 0),

    /*
        <modulus expression> ::= MOD (
             <numeric value expression dividend> ,
             <numeric value expression divisor>
        )
    */
    "ModulusExpression" => SymbolDef::new(&[], 0),

    /*
        <absolute value expression> ::=
            ABS ( <numeric value expression> )
    */
    "AbsoluteValueExpression" => SymbolDef::new(&[], 0),

    /*
        <cardinality expression> ::=
            CARDINALITY ( <collection value expression> )
    */
    "CardinalityExpression" => SymbolDef::new(&[], 0),

    /*
        <extract expression> ::=
            EXTRACT (
                <extract field>
                FROM <extract source>
        )
    */
    "ExtractExpression" => SymbolDef::new(&[], 0),

    /*
        <length expression> ::=
            <char length expression>
     |      <octet length expression>
    */
    "LengthExpression" => SymbolDef::new(&[], 0),

    /*
        <char length expression> ::=
            { CHAR_LENGTH | CHARACTER_LENGTH } (
                <string value expression>
                [ USING <char length units> ]
        )
    */
    "CharLengthExpression" => SymbolDef::new(&[], 0),

    /*
        <octet length expression> ::=
            OCTET_LENGTH ( <string value expression> )
    */
    "OctetLengthExpression" => SymbolDef::new(&[], 0),

    /*
        <position expression>    ::=
            <string position expression>
     |      <blob position expression>
    */
    "PositionExpression" => SymbolDef::new(&[
        "StringPositionExpression",
        "BlobPositionExpression"
    ], 0),
    /*
        <string position expression> ::=
            POSITION (
                <string value expression>
                IN <string value expression>
                [ USING <char length units> ]?
            )
    */
    "StringPositionExpression" => SymbolDef::new(&[], 0),

    /*
        <blob position expression> ::=
            POSITION (
                <blob value expression>
                IN <blob value expression>
            )
    */
    "BlobPositionExpression" => SymbolDef::new(&[], 0),

    /*
        <value expression primary> ::=
            <parenthesized value expression>
            | <nonparenthesized value expression primary>
    */
    "ValueExpressionPrimary" => SymbolDef::new(&[
        "ParenthesizedValueExpression",
        "NonParenthesizedValueExpressionPrimary"
    ], 0),

    /*
        <parenthesized value expression> ::=
            <left paren> <value expression> <right paren>
    */
    "ParenthesizedValueExpression" => SymbolDef::new(&[], 0),

    /*
        <nonparenthesized value expression primary> ::=
            <unsigned value specification>
            | <column reference>
            | <set function specification>
            | <window function>
            | <scalar subquery>
            | <case expression>
            | <cast specification>
            | <field reference>
            | <subtype treatment>
            | <method invocation>
            | <static method invocation>
            | <new specification>
            | <attribute or method reference>
            | <reference resolution>
            | <collection value constructor>
            | <array element reference>
            | <multiset element reference>
            | <routine invocation>
            | <next value expression>
    */
    "NonParenthesizedValueExpressionPrimary" => SymbolDef::new(&[
        "UnsignedValueSpecification",
        "ColumnReference",
        "SetFunctionReference",
        "WindowFunction",
        "ScalarSubquery",
        "CaseExpression",
        "CastSpecification",
        "FieldReference",
        "SubtypeTreatment",
        "MethodInvocation",
        "StaticMethodInvocation",
        "NewSpecification",
        "AttributeOrMethodReference",
        "ReferenceResolution",
        "CollectionValueConstructor",
        "ArrayElementReference",
        "MultisetElementReference",
        "RoutineInvocation",
        "NextValueExpression"
    ], 0),

    /*
        <set function specification> ::=
            <aggregate function>
            | <grouping operation>
    */
    "SetFunctionReference" => SymbolDef::new(&[
        "AggregateFunction",
        "GroupingOperation"
    ], 0),

    /*
        <aggregate function>    ::=
            COUNT <left paren> <asterisk> <right paren> [ <filter clause> ]
            | <general set function> [ <filter clause> ]
            | <binary set function> [ <filter clause> ]
            | <ordered set function> [ <filter clause> ]
    */
    "AggregateFunction" => SymbolDef::new(&[], 0),

    /*
        <unsigned value specification> ::=
            <unsigned literal>
            | <general value specification>
    */
    "UnsignedValueSpecification" => SymbolDef::new(&[
        "UnsignedLiteral",
        "GeneralValueSpecification"
    ], 0),

    /*
        <general value specification>    ::=
            <host parameter specification>
            | <SQL parameter reference>
            | <dynamic parameter specification>
            | <embedded variable specification>
            | <current collation specification>
            | CURRENT_DEFAULT_TRANSFORM_GROUP
            | CURRENT_PATH
            | CURRENT_ROLE
            | CURRENT_TRANSFORM_GROUP_FOR_TYPE <path-resolved user-defined type name>
            | CURRENT_USER
            | SESSION_USER
            | SYSTEM_USER
            | USER
            | VALUE
    */
    "GeneralValueSpecification" => SymbolDef::new(&[
        "HostParameterSpecification",
        "SQLParameterReference",
        "DynamicParameterSpecification",
        "EmbeddedVariableSpecification",
        "CurrentCollationSpecification",
        "CurrentDefaultTransformGroup",
        "CurrentPath",
        "CurrentRole",
        "CurrentTransformGroupForType",
        "CurrentUser",
        "SessionUser",
        "SystemUser",
        "User",
        "Value"
    ], 0),

    /*
        VALUE
    */
    "Value" => SymbolDef::new(&[], 0),

    /*
        USER
    */
    "User" => SymbolDef::new(&[], 0),

    /*
        SESSION_USER
    */
    "SessionUser" => SymbolDef::new(&[], 0),

    /*
        CURRENT_USER
    */
    "CurrentUser" => SymbolDef::new(&[], 0),

    /*
        CURRENT_TRANSFORM_GROUP_FOR_TYPE <path-resolved user-defined type name>
    */
    "CurrentTransformGroupForType" => SymbolDef::new(&[], 0),

    /*
        CURRENT_ROLE
    */
    "CurrentRole" => SymbolDef::new(&[], 0),

    /*
        CURRENT_PATH
    */
    "CurrentPath" => SymbolDef::new(&[], 0),

    /*
        <current collation specification> ::= CURRENT_COLLATION (
            <string value expression>
        )
    */
    "CurrentCollationSpecification" => SymbolDef::new(&[], 0),

    /*
        <host parameter specification> ::=
            <host parameter name>
            [ <indicator parameter> ]
    */
    "HostParameterSpecification" => SymbolDef::new(&["HostParameterName"], 0),

    /*
        <host parameter name> ::= <colon> <identifier>
    */
    "HostParameterName" => SymbolDef::new(&[], 0),

    /*
        <SQL parameter reference> ::= <basic identifier chain>
    */
    "SQLParameterReference" => SymbolDef::new(&["BasicIdentifierChain"], 0),

    /*
        <embedded variable specification> ::= <embedded variable name> [ <indicator variable> ]
     */
    "EmbeddedVariableSpecification" => SymbolDef::new(&["EmbeddedVariableName"], 0),

    /*
        <embedded variable name>    ::=   <colon> <host identifier>
    */
    "EmbeddedVariableName" => SymbolDef::new(&["HostIdentifier"], 0),

    /*
        <host identifier>    ::=
            <Ada host identifier>
            | <C host identifier>
            | <COBOL host identifier>
            | <Fortran host identifier>
            | <MUMPS host identifier>
            | <Pascal host identifier>
            | <PL/I host identifier>
    */
    "HostIdentifier" => SymbolDef::new(&[], 0),

    ///////////////////
    // Literal       //
    ///////////////////
    /*
        <unsigned literal> ::=
            <unsigned numeric literal>
            | <general literal>
    */
    "UnsignedLiteral" => SymbolDef::new(&[
        "UnsignedNumericLiteral",
        "GeneralLiteral"
    ], 0),
    /*
        <literal> ::= <signed numeric literal> | <general literal>
    */
    "Literal" => SymbolDef::new(&[
        "SignedNumericLiteral",
        "GeneralLiteral"
    ], 0),

    /*
        <signed numeric literal> ::=
        [ <sign> ] <unsigned numeric literal>
    */
    "SignedNumericLiteral" => SymbolDef::new(&["UnsignedNumericLiteral"], 0),

    /*
        <unsigned numeric literal> ::=
            <exact numeric literal>
            | <approximate numeric literal>
    */
    "UnsignedNumericLiteral" => SymbolDef::new(&[], 0),

    /*
        <general literal> ::=
            <character string literal>
            | <national character string literal>
            | <Unicode character string literal>
            | <binary string literal>
            | <datetime literal>
            | <interval literal>
            | <boolean literal>
    */
    "GeneralLiteral" => SymbolDef::new(&[
        "CharacterStringLiteral"
    ], 0),

    /*
        <character string literal>    ::=
         [ <introducer> <character set specification> ]
         ' [ <character representation> ... ] '
         [ { <separator> ' [ <character representation> ... ] ' }... ]
    */
    "CharacterStringLiteral" => SymbolDef::new(&[], 0),



    ///////////////////////////
    // Names and identifiers //
    ///////////////////////////

    /*
        <identifier> ::= <actual identifier>
    */
    "Identifier" => SymbolDef::new(&[], 0),

    /*
        <schema name> ::=
            [ <catalog name> <period> ]
            <unqualified schema name>
    */
    "SchemaName" => SymbolDef::new(&["UnqualifiedSchemaName"], 0),

    /*
        <unqualified schema name> ::= <identifier>
    */
    "UnqualifiedSchemaName" => SymbolDef::new(&["Identifier"], 0),

    /*
        <basic identifier chain> ::= <identifier chain>
    */
    "BasicIdentifierChain" => SymbolDef::new(&["IdentifierChain"], 0),
    /*
        <identifier chain> ::=
            <identifier> [ { . <identifier> }... ]
    */
    "IdentifierChain" => SymbolDef::new(&["Identifier"], WITH_HELPERS),

    /*
        <column reference> ::=
            <basic identifier chain>
            | MODULE <period> <qualified identifier> <period> <column name>
    */
    "ColumnReference" => SymbolDef::new(&["BasicIdentifierChain"], 0),

    /*
        <column name list> ::= <column name> [ { <comma> <column name> }... ]
    */
    "ColumnNameList" => SymbolDef::new(&["ColumnName"], WITH_HELPERS),
    /*
        <column name> ::= <identifier>
    */
    "ColumnName" => SymbolDef::new(&["Identifier"], 0),

    /*
        <table reference list> ::= <table reference> [ { <comma> <table reference> }... ]
    */
    "TableReferenceList" => SymbolDef::new(&[
        "TableReference"
    ], WITH_HELPERS),

    /*
        <table reference> ::= <table primary or joined table> [ <sample clause> ]
    */
    "TableReference" => SymbolDef::new(&[
        "TablePrimaryOrJoinedTable"
    ], WITH_HELPERS),

    /*
        <table primary or joined table> ::=
            <table primary>
            | <joined table>
    */
    "TablePrimaryOrJoinedTable" => SymbolDef::new(&[
        "TablePrimary",
        "JoinedTable"
    ], 0),

    /*
        <table primary>    ::=
            <table or query name> [ [ AS ] <correlation name> [ <left paren> <derived column list> <right paren> ] ]
            | <derived table> [ AS ] <correlation name> [ <left paren> <derived column list> <right paren> ]
            | <lateral derived table> [ AS ] <correlation name> [ <left paren> <derived column list> <right paren> ]
            | <collection derived table> [ AS ] <correlation name> [ <left paren> <derived column list> <right paren> ]
            | <table function derived table> [ AS ] <correlation name> [ <left paren> <derived column list> <right paren> ]
            | <only spec> [ [ AS ] <correlation name> [ <left paren> <derived column list> <right paren> ] ]
            | <parenthesized joined table>
     */
    "TablePrimary" => SymbolDef::new(&[
        "TableOrQueryName",
        "DerivedTable",
        "LateralDerivedTable",
        "CollectionDerivedTable",
        "TableFunctionDerivedTable",
        "OnlySpec",
        "ParenthesizedJoinedTable"
    ], 0),

    /*
        <table or query name> ::= <table name> | <query name>
    */
    "TableOrQueryName" => SymbolDef::new(&[
        "TableName",
        "QueryName"
    ], 0),

    /*
        <lateral derived table> ::= LATERAL <table subquery>
    */
    "LateralDerivedTable" => SymbolDef::new(&["TableSubquery"], 0),

    /*
        <collection derived table> ::= UNNEST
            <left paren> <collection value expression> <right paren>
            [ WITH ORDINALITY ]
    */
    "CollectionDerivedTable" => SymbolDef::new(&[], 0),

    /*
        <only spec> ::= ONLY
        <left paren> <table or query name> <right paren>
    */
    "OnlySpec" => SymbolDef::new(&[], 0),

    /*
        <table function derived table> ::= TABLE
            <left paren> <collection value expression> <right paren>
    */
    "TableFunctionDerivedTable" => SymbolDef::new(&[], 0),

    /*
        <left paren> <joined table> <right paren>
     */
    "ParenthesizedJoinedTable" => SymbolDef::new(&[], 0),

    /*
        <derived table> ::= <table subquery>
     */
    "DerivedTable" => SymbolDef::new(&["TableSubquery"], 0),
    /*
        <table subquery> ::= <subquery>
     */
    "TableSubquery" => SymbolDef::new(&["Subquery"], 0),

    /*
        <subquery> ::= <left paren> <query expression> <right paren>
    */
    "Subquery" => SymbolDef::new(&[], 0),

    /*
        <table name> ::= <local or schema qualified name>
    */
    "TableName" => SymbolDef::new(&[
        "LocalOrSchemaQualifiedName"
    ], 0),
    /*
        <local or schema qualified name> ::=
        [ <local or schema qualifier> <period> ] <qualified identifier>
    */
    "LocalOrSchemaQualifiedName" => SymbolDef::new(&[
        "QualifiedIdentifier"
    ], 0),

    /*
        <local or schema qualifier> ::= <schema name> | MODULE
    */
    "LocalOrSchemaQualifier" => SymbolDef::new(&["SchemaName", "Module"], 0),
    /*
        MODULE
    */
    "Module" => SymbolDef::new(&[], 0),

    /*
        <qualified identifier> ::= <identifier>
    */
    "QualifiedIdentifier" => SymbolDef::new(&["Identifier"], 0),
    "Qualifier" => SymbolDef::new(&[], 0),

    "Insert" => SymbolDef::new(&[], WITH_HELPERS | WITH_REQUIRED_HELPERS_METHOD),
    "InsertionTarget" => SymbolDef::new(&["TableName"], 0),
    "InsertColumnsAndSources" => SymbolDef::new(&[
        "FromSubQuery",
        "FromConstructor",
        "FromDefault"
    ], WITH_BLANK_IMPL),
    "FromSubQuery" => SymbolDef::new(&[], 0),
    "FromConstructor" => SymbolDef::new(&[
        "ContextuallyTypedTableValueConstructor"
    ], 0),
    "InsertColumnList" => SymbolDef::new(&["ColumnNameList"], WITH_BLANK_IMPL),
    "OverrideClause" => SymbolDef::new(&["OverridingUserValue", "OverridingSystemValue"], WITH_BLANK_IMPL),
    "OverridingUserValue" => SymbolDef::new(&[], 0),
    "OverridingSystemValue" => SymbolDef::new(&[], 0),

    "FromDefault" => SymbolDef::new(&[], 0),

    "ContextuallyTypedTableValueConstructor" => SymbolDef::new(&["ContextuallyTypedRowValueExpressionList"], 0),
    "ContextuallyTypedRowValueExpressionList" => SymbolDef::new(&["ContextuallyTypedRowValueExpression"], WITH_HELPERS),
    "ContextuallyTypedRowValueExpression" => SymbolDef::new(&[
        "ContextuallyTypedRowValueConstructor"
    ], 0),

    /*
        <contextually typed row value constructor>    ::=
            <common value expression>
            | <boolean value expression>
            | <contextually typed value specification>
            | ( <contextually typed row value constructor element list> )
            | ROW ( <contextually typed row value constructor element list> )
    */
    "ContextuallyTypedRowValueConstructor" => SymbolDef::new(&[
        "CommonValueExpression",
        "BooleanValueExpression",
        "ContextuallyTypedRowValueConstructorElementList"
    ], 0),

    /*
        <row value constructor element list>    ::=
            <row value constructor element>
            | <row value constructor element list>, <row value constructor element>
    */
    "ContextuallyTypedRowValueConstructorElementList" => SymbolDef::new(&[
        "ContextuallyTypedRowValueConstructorElement"
    ], WITH_HELPERS),

    /*
        <contextually typed row value constructor element> ::=
            <value expression>
            | <contextually typed value specification>
    */
    "ContextuallyTypedRowValueConstructorElement" => SymbolDef::new(&[
        "ValueExpression"
    ], 0),

    "ValueSpecification" => SymbolDef::new(&[], 0),

    /*
        <search condition> ::= <boolean value expression>
    */
    "SearchCondition" => SymbolDef::new(&["BooleanValueExpression"], WITH_HELPERS),
    /*
        <boolean value expression> ::=
            <boolean term>
            | <boolean value expression> OR <boolean term>
    */
    "BooleanValueExpression" => SymbolDef::new(&["BooleanTerm"], 0),
    /*
        <boolean term> ::=
            <boolean factor>
            | <boolean term> AND <boolean factor>
    */
    "BooleanTerm" => SymbolDef::new(&["BooleanFactor"], 0),
    /*
        <boolean factor> ::= [ NOT ] <boolean test>
    */
    "BooleanFactor" => SymbolDef::new(&["BooleanTest"], 0),
    /*
        <boolean test> ::= <boolean primary> [ IS [ NOT ] <truth value> ]
    */
    "BooleanTest" => SymbolDef::new(&["BooleanPrimary"], 0),
    /*
        <truth value>    ::=   TRUE | FALSE | UNKNOWN
    */
    "TruthValue" => SymbolDef::new(&[], 0),
    /*
        <boolean primary> ::=
            <predicate>
            | <boolean predicand>
    */
    "BooleanPrimary" => SymbolDef::new(&[
        "Predicate",
        "BooleanPredicand"
    ], 0),

    /*
        <boolean predicand>    ::=
            <parenthesized boolean value expression>
            | <nonparenthesized value expression primary>
    */
    "BooleanPredicand" => SymbolDef::new(&[
        "ParenthesizedBooleanValueExpression",
        "NonparenthesizedValueExpressionPrimary"
    ], 0),

    /*
        <parenthesized boolean value expression> ::=
            ( <boolean value expression> )
    */
    "ParenthesizedBooleanValueExpression" => SymbolDef::new(&[], 0),

    /*
        <predicate>    ::=
            <comparison predicate>
            | <between predicate>
            | <in predicate>
            | <like predicate>
            | <similar predicate>
            | <null predicate>
            | <quantified comparison predicate>
            | <exists predicate>
            | <unique predicate>
            | <normalized predicate>
            | <match predicate>
            | <overlaps predicate>
            | <distinct predicate>
            | <member predicate>
            | <submultiset predicate>
            | <set predicate>
            | <type predicate>
    */
    "Predicate" => SymbolDef::new(&[
        "ComparisonPredicate",
        "BetweenPredicate",
        "InPredicate",
        "LikePredicate",
        "SimilarPredicate",
        "NullPredicate",
        "QuantifiedComparisonPredicate",
        "ExistsPredicate",
        "UniquePredicate",
        "NormalizedPredicate",
        "MatchPredicate",
        "OverlapsPredicate",
        "DistinctPredicate",
        "MemberPredicate",
        "SubmultisetPredicate",
        "SetPredicate",
        "TypePredicate"
    ], 0),

    /*
        <comparison predicate> ::=
            <row value predicand>
            <comparison predicate part 2>
    */
    "ComparisonPredicate" => SymbolDef::new(&[], 0),
    /*
        <between predicate> ::=
            <row value predicand>
            <between predicate part 2>
    */
    "BetweenPredicate" => SymbolDef::new(&[], 0),
    /*
        <in predicate> ::=
            <row value predicand>
            <in predicate part 2>
    */
    "InPredicate" => SymbolDef::new(&[], 0),
    /*
        <like predicate> ::=
            <character like predicate>
            | <octet like predicate>
    */
    "LikePredicate" => SymbolDef::new(&[], 0),
    /*
        <similar predicate> ::=
            <row value predicand>
            <similar predicate part 2>
    */
    "SimilarPredicate" => SymbolDef::new(&[], 0),
    /*
        <null predicate> ::=
            <row value predicand>
            <null predicate part 2>
    */
    "NullPredicate" => SymbolDef::new(&[], 0),
    /*
        <quantified comparison predicate> ::=
            <row value predicand>
            <quantified comparison predicate part 2>
    */
    "QuantifiedComparisonPredicate" => SymbolDef::new(&[], 0),
    /*
        <exists predicate> ::= EXISTS <table subquery>
    */
    "ExistsPredicate" => SymbolDef::new(&[], 0),
    /*
        <unique predicate> ::= UNIQUE <table subquery>
    */
    "UniquePredicate" => SymbolDef::new(&[], 0),
    /*
        <normalized predicate> ::=
            <string value expression>
            IS [ NOT ] NORMALIZED
    */
    "NormalizedPredicate" => SymbolDef::new(&[], 0),
    /*
        <match predicate> ::=
            <row value predicand>
            <match predicate part 2>
    */
    "MatchPredicate" => SymbolDef::new(&[], 0),
    /*
        <overlaps predicate> ::=
            <overlaps predicate part 1>
            <overlaps predicate part 2>
    */
    "OverlapsPredicate" => SymbolDef::new(&[], 0),

    /*
        <distinct predicate> ::=
            <row value predicand 3>
            <distinct predicate part 2>
    */
    "DistinctPredicate" => SymbolDef::new(&[], 0),
    /*
        <member predicate> ::=
            <row value predicand>
            <member predicate part 2>
    */
    "MemberPredicate" => SymbolDef::new(&[], 0),
    /*
        <submultiset predicate> ::=
            <row value predicand>
            <submultiset predicate part 2>
    */
    "SubmultisetPredicate" => SymbolDef::new(&[], 0),
    /*
        <set predicate> ::=
            <row value predicand>
            <set predicate part 2>
    */
    "SetPredicate" => SymbolDef::new(&[], 0),

    /*
        <type predicate> ::=
            <row value predicand>
            <type predicate part 2>
    */
    "TypePredicate" => SymbolDef::new(&[], 0),

    /*
        <row value predicand>    ::=
            <row value special case>
            | <row value constructor predicand>
    */
    "RowValuePredicand" => SymbolDef::new(&[
        "RowValueSpecialCase",
        "RowValueConstructorPredicand"
    ], 0),

    /*
        <row value special case> ::=
            <nonparenthesized value expression primary>
    */
    "RowValueSpecialCase" => SymbolDef::new(&[
        "NonparenthesizedValueExpressionPrimary"
    ], 0),

    /*
        <row value constructor predicand>    ::=
            <common value expression>
            | <boolean predicand>
            | <explicit row value constructor>
    */
    "RowValueConstructorPredicand" => SymbolDef::new(&[
        "CommonValueExpression",
        "BooleanPredicand",
        "ExplicitRowValueConstructor"
    ], 0),

    /*
        <explicit row value constructor>    ::=
            (<row value constructor element>, <row value constructor element list>)
        | ROW <left paren> <row value constructor element list> <right paren>
        | <row subquery>
    */
    "ExplicitRowValueConstructor" => SymbolDef::new(&["RowSubquery"], 0),
    /*
        <row subquery> ::=   <subquery>
    */
    "RowSubquery" => SymbolDef::new(&["Subquery"], 0),
    /*
        <row value constructor>    ::=
            <common value expression>
            | <boolean value expression>
            | <explicit row value constructor>
    */
    "RowValueConstructor" => SymbolDef::new(&[
        "CommonValueExpression",
        "BooleanValueExpression",
        "ExplicitRowValueConstructor"
    ], 0),
    "RowValueConstructorList" => SymbolDef::new(&[], 0),
    "RowValueConstructorElement" => SymbolDef::new(&[], 0),
    "RowValueExpression" => SymbolDef::new(&[], 0),

    "WindowFunction" => SymbolDef::new(&[], 0),
    "NonparenthesizedValueExpressionPrimary" => SymbolDef::new(&[], 0),
    "DynamicParameterSpecification" => SymbolDef::new(&[], 0),
    "QueryName" => SymbolDef::new(&[], 0),
    "CurrentDefaultTransformGroup" => SymbolDef::new(&[], 0),
    "GroupingOperation" => SymbolDef::new(&[], 0),
    "SystemUser" => SymbolDef::new(&[], 0),
    "ScalarSubquery" => SymbolDef::new(&[], 0),
    "CaseExpression" => SymbolDef::new(&[], 0),
    "CastSpecification" => SymbolDef::new(&[], 0),
    "FieldReference" => SymbolDef::new(&[], 0),
    "SubtypeTreatment" => SymbolDef::new(&[], 0),
    "MethodInvocation" => SymbolDef::new(&[], 0),
    "StaticMethodInvocation" => SymbolDef::new(&[], 0),
    "NewSpecification" => SymbolDef::new(&[], 0),
    "AttributeOrMethodReference" => SymbolDef::new(&[], 0),
    "ReferenceResolution" => SymbolDef::new(&[], 0),
    "CollectionValueConstructor" => SymbolDef::new(&[], 0),
    "ArrayElementReference" => SymbolDef::new(&[], 0),
    "MultisetElementReference" => SymbolDef::new(&[], 0),
    // <routine invocation>  ::=<routine name> <SQL argument list>
    "RoutineInvocation" => SymbolDef::new(&[], 0),
    // <routine name> ::= [ <schema name> <period> ] <qualified identifier>
    "RoutineName" => SymbolDef::new(&["QualifiedIdentifier"], 0),
    // <SQL argument list> ::= <left paren> [ <SQL argument> [ { <comma> <SQL argument> }... ] ] <right paren>
    "SQLArgumentList" => SymbolDef::new(&["SQLArgument"], WITH_BLANK_IMPL | WITH_HELPERS ),
    /*
        <SQL argument>    ::=
            <value expression>
            | <generalized expression>
            | <target specification>
    */
    "SQLArgument" => SymbolDef::new(&["ValueExpression"], 0),
    "NextValueExpression" => SymbolDef::new(&[], 0),
};

pub struct SymbolDependentsIterator<'s> {
    stack: Vec<&'s str>,
    visited: Vec<&'s str>,
}

impl<'s> Iterator for SymbolDependentsIterator<'s> {
    type Item = &'s str;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(symbol) = self.stack.pop() {
            if !self.visited.contains(&symbol) {
                self.stack.extend(
                    SYMBOL_MAP
                        .entries()
                        .filter(move |(_, def)| def.deps.contains(&symbol))
                        .map(|(key, _)| key),
                );
            }

            self.visited.push(symbol);
            return Some(symbol);
        }

        None
    }
}

/// Iterate over all dependents of the symbol.
///
/// The iterator does not prevent loops.
pub fn iter_dependents(symbol: &str) -> impl Iterator<Item = &'_ str> {
    SymbolDependentsIterator {
        stack: vec![symbol],
        visited: vec![],
    }
}

pub fn detect_loop(symbol: &str) -> Result<(), Vec<&str>> {
    let mut acc = vec![symbol];

    for el in iter_dependents(symbol).dropping(1) {
        acc.push(el);

        if el == symbol {
            return Err(acc);
        }
    }

    Ok(())
}

/// Fetch the dependents of the symbol.
pub fn fetch_deps(symbol: &str) -> impl Iterator<Item = &str> {
    iter_dependents(symbol)
        .unique()
        .filter(move |&dep| dep != symbol)
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use crate::fetch_deps;

    #[test]
    fn test_from_clause_dependents() {
        let deps = fetch_deps("FromClause").collect_vec();
        let dep = "TableExpression";
        assert!(deps.contains(&dep), "{:?}", deps)
    }
}
