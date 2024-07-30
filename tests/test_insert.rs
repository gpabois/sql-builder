use sql_builder::{bind, id, insert, lit, prelude::*};

#[test]
pub fn test_insert_simple() {
    let bound = bind(10);
    let values = lit!(10)
        .add_row_element(lit!(20))
        .add_row_element(bound)
        .into_row_value()
        .add_row_value(lit!(20).into_row_value());

    let stmt = insert(id!(my_table))
        .columns(
            id!(col1)
                .add_column(id!(col2))
                .add_column(id!(col3))
                .transform_if(false, |cols| cols.add_column(id!(col4))),
        )
        .values(values);

    println!("{}", stmt);
    assert!(false)
}
