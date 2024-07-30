use sql_builder::{bind, columns, id, insert, lit, prelude::*, row_value};

#[test]
pub fn test_insert_simple() {
    let values = row_value!(lit!(10), lit!(20), bind(10));
    let table = id!(my_table);
    let columns = columns!(id!(col1), id!(col2), id!(col3))
        .transform_if(false, |kernel| columns!(kernel, id!(col4)));

    let stmt = insert(table).columns(columns).values(values);

    println!("{}", stmt);
    assert!(false)
}
