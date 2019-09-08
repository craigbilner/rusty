use spiral2;

#[test]
fn test_1() {
    assert_eq!(spiral2::greater_than(1), 2);
}

#[test]
fn test_11() {
    assert_eq!(spiral2::greater_than(11), 23);
}

#[test]
fn test_54() {
    assert_eq!(spiral2::greater_than(54), 57);
}

#[test]
fn test_747() {
    assert_eq!(spiral2::greater_than(747), 806);
}
