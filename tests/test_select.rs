use sql_builder::{eq, id, lit, or, select, select_columns};
use sql_builder::{lt, prelude::*};

#[test]
fn test_select_basic() {
    let selected_columns = select_columns!(
        id!(col1),
        id!(col2).alias_column(id!(aliased_column)),
        id!(col3)
    );

    let table = id!(table);
    let stmt = select(selected_columns).from(table);

    let sql = stmt.to_string();
    assert_eq!(
        sql,
        "SELECT col1, col2 AS aliased_column, col3 FROM my_table"
    );
}

#[test]
fn test_select_where() {
    let selected_columns = select_columns!(id!(col1), id!(col2));
    let table = id!(my_table);
    let cond = or(eq(id!(col1), lit!(10)), lt(id!(col2), lit!(20)));
    let stmt = select(selected_columns).from(table).r#where(cond);

    let sql = stmt.to_string();
    assert_eq!(
        sql,
        "SELECT col1, col2 FROM my_table WHERE col1 = 10 OR col2 < 20"
    );
}

#[test]
fn test_select_distinct() {
    let stmt = select(id("col1")).from(id("my_table")).distinct();
    let sql = stmt.to_string();
    assert_eq!(sql, "SELECT DISTINCT col1 FROM my_table");
}
