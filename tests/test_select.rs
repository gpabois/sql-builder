use sql_builder::{grammar::SelectList, ToQuery};
pub use sql_builder::{id, select};

#[test]
/// Simple select building.
fn test_select_simple() {
    let columns = id!(col_1).append(id!(col_2));
    let sel = select(columns).from(id! {my_table});

    let sql = sel.to_string().unwrap();

    assert_eq!(sql, "SELECT col_1, col_2 FROM my_table");
}
