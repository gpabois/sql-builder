use sql_builder::{eq, helpers::TableReference, id, prelude::*};

#[test]
fn test_inner_join() {
    let joined_table = id!(joinee)
        .inner_join(id!(joiner))
        .on(eq(id!(joinee.fk_id), id!(joiner.id)));

    let sql = joined_table.to_string();
    assert_eq!(sql, "joinee INNER JOIN joiner ON joinee.fk_id = joiner.id")
}
