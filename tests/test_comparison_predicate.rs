use sql_builder::{eq, gt, gte, id, lit, lt, lte, neq};

#[test]
fn test_eq() {
    let lhs = id!(test);
    let rhs = lit!(10.123);

    let cmp = eq(lhs, rhs);
    let sql = cmp.to_string();
    assert_eq!(sql, "test = 10.123");
}

#[test]
fn test_neq() {
    let lhs = id!(test);
    let rhs = lit!(10.123);

    let cmp = neq(lhs, rhs);
    let sql = cmp.to_string();
    assert_eq!(sql, "test <> 10.123");
}

#[test]
fn test_lt() {
    let lhs = id!(test);
    let rhs = lit!(10.123);
    let cmp = lt(lhs, rhs);
    let sql = cmp.to_string();
    assert_eq!(sql, "test < 10.123");
}

#[test]
fn test_lte() {
    let lhs = id!(test);
    let rhs = lit!(10.123);
    let cmp = lte(lhs, rhs);
    let sql = cmp.to_string();
    assert_eq!(sql, "test <= 10.123");
}

#[test]
fn test_gt() {
    let lhs = id!(test);
    let rhs = lit!(10.123);
    let cmp = gt(lhs, rhs);
    let sql = cmp.to_string();
    assert_eq!(sql, "test > 10.123");
}

#[test]
fn test_gte() {
    let lhs = id!(test);
    let rhs = lit!(10.123);
    let cmp = gte(lhs, rhs);
    let sql = cmp.to_string();
    assert_eq!(sql, "test >= 10.123");
}

