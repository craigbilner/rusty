use look_and_say;

#[test]
fn test_1() {
    assert_eq!(look_and_say::length("1", 1), 2);
}

#[test]
fn test_2() {
    assert_eq!(look_and_say::length("1", 2), 2);
}

#[test]
fn test_3() {
    assert_eq!(look_and_say::length("1", 3), 4);
}

#[test]
fn test_4() {
    assert_eq!(look_and_say::length("1", 4), 6);
}

#[test]
fn test_5() {
    assert_eq!(look_and_say::length("1", 5), 6);
}