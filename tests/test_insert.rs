use sql_builder::{id, insert};

#[test]
pub fn test_insert_simple() {
    let stmt = insert(id!(my_table));
}