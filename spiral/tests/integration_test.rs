use spiral;

/*
Data from square 1 is carried 0 steps, since it's at the access port.
Data from square 12 is carried 3 steps, such as: down, left, left.
Data from square 23 is carried only 2 steps: up twice.
Data from square 1024 must be carried 31 steps.
*/

#[test]
fn test_1() {
    assert_eq!(spiral::manhattan_distance(1), 0);
}

#[test]
fn test_12() {
    assert_eq!(spiral::manhattan_distance(12), 3);
}

#[test]
fn test_23() {
    assert_eq!(spiral::manhattan_distance(23), 2);
}

#[test]
fn test_1024() {
    assert_eq!(spiral::manhattan_distance(1024), 31);
}