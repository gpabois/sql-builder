use sql_builder::{id, invoke, lit, prelude::*};

#[test]
fn test_routine_invocation_no_args() {
    let call = invoke!(NOW());
    let sql = call.to_string();
    assert_eq!(sql, "NOW()")
}

#[test]
fn test_routine_invocation_with_args() {
    let call = invoke!(FUNC(id!(foo), lit!(10)));
    let sql = call.to_string();
    assert_eq!(sql, "FUNC(foo, 10)")
}
