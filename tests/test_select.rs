use sql_builder::{helpers::{QuerySpecification as _, SelectSublist, ValueExpression as _}, ToQuery};
pub use sql_builder::{id, select};

#[test]
fn test_select_basic() {
    let selected_columns = id("col1")
        .add_selection(id("col2").alias_column(id("aliased_column")))
        .add_selection(id("col3"));

    let table = id("my_table");
    let stmt = select(selected_columns).from(table);

    let sql = stmt.to_raw_query().unwrap();
    assert_eq!(
        sql,
        "SELECT col1, col2 AS aliased_column, col3 FROM my_table"
    );
}

#[test]
fn test_select_distinct() {
    let stmt = select(id("col1")).from(id("my_table")).distinct();
    let sql = stmt.to_raw_query().unwrap();
    assert_eq!(sql, "SELECT DISTINCT col1 FROM my_table");
}