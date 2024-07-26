use sql_builder::{helpers::{ColumnNameList, ValueExpression}, id, insert, Symbol};

#[test]
pub fn test_insert_simple() {
    let stmt = insert(id!(my_table))
        .columns(
            id!(col1)
            .add_column(id!(col2))
            .add_column(id!(col3))
            .transform_if(false, 
                |cols| 
                    cols.add_column(id!(col4))
            )
        );
}