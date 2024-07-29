use sql_builder::{eq, id, lit, select};
use sql_builder::{lt, prelude::*};

#[test]
fn test_select_basic() {
    let selected_columns = id("col1")
        .add_selection(id("col2").alias_column(id("aliased_column")))
        .add_selection(id("col3"));

    let table = id("my_table");
    let stmt = select(selected_columns).from(table);

    let sql = stmt.to_string();
    assert_eq!(
        sql,
        "SELECT col1, col2 AS aliased_column, col3 FROM my_table"
    );
}

#[test]
fn test_select_where() {
    let cond = eq(id!(col1), lit!(10)).or(lt(id!(col2), lit!(20)));
    let stmt = select(id!(col1).add_selection(id!(col2)))
        .from(id!(my_table))
        .r#where(cond);

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
