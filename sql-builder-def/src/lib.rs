use phf::phf_map;

pub struct SymbolDef {
    flags: u8,
    deps: &'static [&'static str],
}

impl SymbolDef {
    #[inline]
    pub const fn new(
        deps: &'static [&'static str],
        flags: u8
    ) -> Self {
        Self {
            flags,
            deps,
        }
    }

    #[inline]
    pub fn with_blank_impl(&self) -> bool {
        self.flags & WITH_BLANK_IMPL == WITH_BLANK_IMPL
    }

    pub fn with_helpers(&self) -> bool {
        self.flags & WITH_HELPERS == WITH_HELPERS
    }
}

pub const WITH_BLANK_IMPL: u8 = 0b1;
pub const WITH_HELPERS: u8 = 0b10;

/// Defines the way how symbols are derived.
static SYMBOL_MAP: phf::Map<&'static str, SymbolDef> = phf_map! {
    "Asterisk" => SymbolDef::new(&[], 0),

    /*
        <query specification> ::= 
            SELECT [ <set quantifier> ] 
                <select list> 
                <table expression>
    */
    "QuerySpecification" => SymbolDef::new(&[], WITH_HELPERS),
    /*
        <table expression>    ::=
            <from clause>
            [ <where clause> ]
            [ <group by clause> ]
            [ <having clause> ]
            [ <window clause> ] 
    */
    "TableExpression" => SymbolDef::new(&["FromClause"], WITH_HELPERS),

    /*
        <from clause> ::= FROM <table reference list>
    */
    "FromClause" => SymbolDef::new(&[], WITH_HELPERS),
    /* 
        <where clause> ::= WHERE <search condition>
    */
    "WhereClause" => SymbolDef::new(&[], WITH_BLANK_IMPL),
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
            <derived column> 
            | <qualified asterisk>
            | <select sublist> [ { <comma> <select sublist> }... ]

        Difference with the ISO/IEC 9075-2:2003 :
        Recursive <select sublist> chaining.
    */
    "SelectSublist" => SymbolDef::new(&[
        "DerivedColumn", 
        "QualifiedAsterisk"
    ], WITH_BLANK_IMPL),
    
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
    ], 0),

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
    ], WITH_HELPERS),

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
    "Term" => SymbolDef::new(&["NumericValueExpression"], 0),

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
    "UnsignedValueSpecification" => SymbolDef::new(&[], 0),

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

    /* 
        <unsigned literal> ::= 
            <unsigned numeric literal> 
            | <general literal>
    */
    "UnsignedLiteral" => SymbolDef::new(&[], 0),

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
    "GeneralLiteral" => SymbolDef::new(&[], 0),

    /*
        <column reference> ::=
            <basic identifier chain>
            | MODULE <period> <qualified identifier> <period> <column name>
    */
    "ColumnReference" => SymbolDef::new(&[
        "BasicIdentifierChain"
    ], 0),

    /*
        <column name> ::= <identifier>
    */
    "ColumnName" => SymbolDef::new(&[
        "Identifier"
    ], 0),

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
    ], 0),

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

    "Qualifier" => SymbolDef {
        with_blank_impl: false,
        deps: &["ColumnReference", "SelectList"]
    },
    "Insert" => SymbolDef {
        with_blank_impl: false,
        deps: &[]
    },
    "InsertionTarget" => SymbolDef {
        with_blank_impl: false,
        deps: &[]
    },
    "InsertColumnsAndSources" => SymbolDef {
        with_blank_impl: false,
        deps: &[]
    },
    "FromSubQuery" => SymbolDef {
        with_blank_impl: false,
        deps: &["InsertColumnsAndSources"]
    },
    "FromConstructor" => SymbolDef {
        with_blank_impl: false,
        deps: &["InsertColumnsAndSources"]
    },
    "FromDefault" => SymbolDef {
        with_blank_impl: false,
        deps: &["InsertColumnsAndSources"]
    },
    "ContextuallyTypedTableValueConstructor" => SymbolDef {
        with_blank_impl: false,
        deps: &["FromConstructor"]
    },
    "ContextuallyTypedRowValueExpressionList" => SymbolDef {
        with_blank_impl: false,
        deps: &[]
    },
    "ContextuallyTypedRowValueExpression" => SymbolDef {
        with_blank_impl: false,
        deps: &["ContextuallyTypedRowValueExpressionList"]
    },
    "ContextuallyTypedRowValueConstructor" => SymbolDef {
        with_blank_impl: false,
        deps: &["ContextuallyTypedRowValueExpression"]
    },
    "ContextuallyTypedRowValueConstructorElementList" => SymbolDef {
        with_blank_impl: false,
        deps: &[
            "ContextuallyTypedRowValueConstructor"
        ]
    },
    "ContextuallyTypedRowValueConstructorElement" => SymbolDef {
        with_blank_impl: false,
        deps: &[
            "ContextuallyTypedRowValueConstructor",
            "ContextuallyTypedRowValueConstructorElementList"
        ]
    },
       "ValueSpecification" => SymbolDef{
        with_blank_impl: false,
        deps: &["ValueExpressionPrimary"]
    },
    "Literal" => SymbolDef{
        with_blank_impl: false,
        deps: &["ValueSpecification"]
    },
    "SchemaName" => SymbolDef{
        with_blank_impl: false,
        deps: &[]
    },
    "UnqualifiedSchemaName" => SymbolDef{
        with_blank_impl: false,
        deps: &["SchemaName"]
    },
    "QualifiedName" => SymbolDef{
        with_blank_impl: false,
        deps: &["TableName"]
    },

    "BasicIdentifier" => SymbolDef::new(&["IdentifierChain"], 0),
    "IdentifierChain" => SymbolDef::new(&["Identifier"], WITH_HELPERS),

    "Identifier" => SymbolDef{
        with_blank_impl: false,
        deps: &[
            "QualifiedIdentifier",
            "ColumnName"
        ]
    },
    "SearchCondition" => SymbolDef {
        with_blank_impl: false,
        deps: &[]
    },
    "BooleanTerm" => SymbolDef {
        with_blank_impl: false,
        deps: &["SearchCondition"]
    },
    "BooleanFactor" => SymbolDef {
        with_blank_impl: false,
        deps: &["BooleanTerm"]
    },
    "BooleanTest" => SymbolDef {
        with_blank_impl: false,
        deps: &["BooleanFactor"]
    },
    "BooleanPrimary" => SymbolDef {
        with_blank_impl: false,
        deps: &["BooleanTest"]
    },

    "Predicate" => SymbolDef {
        with_blank_impl: false,
        deps: &["BooleanPrimary"]
    },

    "ComparisonPredicate" => SymbolDef {
        with_blank_impl: false,
        deps: &["Predicate"]
    },
    "BetweenPredicate" => SymbolDef {
        with_blank_impl: false,
        deps: &["Predicate"]
    },
    "InPredicate" => SymbolDef {
        with_blank_impl: false,
        deps: &["Predicate"]
    },
    "LikePredicate" => SymbolDef {
        with_blank_impl: false,
        deps: &["Predicate"]
    },
    "NullPredicate" => SymbolDef {
        with_blank_impl: false,
        deps: &["Predicate"]
    },
    "ExistsPredicate" => SymbolDef {
        with_blank_impl: false,
        deps: &["Predicate"]
    },
    "MatchPredicate" => SymbolDef {
        with_blank_impl: false,
        deps: &["Predicate"]
    },
    "OverlapsPredicate" => SymbolDef {
        with_blank_impl: false,
        deps: &["Predicate"]
    },

    "RowValueConstructor" => SymbolDef {
        with_blank_impl: false,
        deps: &[]
    },
    "RowValueConstructorList" => SymbolDef {
        with_blank_impl: false,
        deps: &["RowValueConstructor"]
    },
    "RowValueConstructorElement" => SymbolDef {
        with_blank_impl: false,
        deps: &["RowValueConstructor", "RowValueConstructorList"]
    },

};

/// Fetch the dependents of the symbol. 
pub fn fetch_deps(symbol: &str) -> Vec<&str> {
    let mut stack = vec![symbol];
    let mut symbols = Vec::<&'static str>::default();
    while let Some(symbol) = stack.pop() {
        if symbols.contains(&symbol) {
            continue;
        }
        symbols.push(symbol);

        stack.extend(SYMBOL_MAP.keys().find(move |key| {
            let flags = &SYMBOL_MAP[**key];
            flags.deps.contains(&symbol)
        }));
    }
    symbols
}