use std::collections::HashMap;

enum Direction {
    Right,
    Up,
    Left,
    Down,
}

pub fn greater_than(val: u32) -> i32 {
    let mut squares = HashMap::new();
    let mut dir = Direction::Right;
    let mut x = 0;
    let mut y = 0;
    let mut layer = 0;

    squares.insert(format!("{}|{}", x, y), 1);
    let mut current_value = 1;

    loop {
        if current_value > val as i32 {
            return current_value;
        }

        match dir {
            Direction::Right => {
                x = x + 1;

                if x > layer {
                    layer = layer + 1;
                    dir = Direction::Up;
                }

                current_value = sum_surrounds(&squares, x, y);

                squares.insert(format!("{}|{}", x, y), current_value);
            }
            Direction::Up => {
                y = y + 1;

                current_value = sum_surrounds(&squares, x, y);

                squares.insert(format!("{}|{}", x, y), current_value);

                if y == layer {
                    dir = Direction::Left;
                }
            }
            Direction::Left => {
                x = x - 1;

                current_value = sum_surrounds(&squares, x, y);

                squares.insert(format!("{}|{}", x, y), current_value);

                if x.abs() == layer {
                    dir = Direction::Down;
                }
            }
            Direction::Down => {
                y = y - 1;

                current_value = sum_surrounds(&squares, x, y);

                squares.insert(format!("{}|{}", x, y), current_value);

                if y.abs() == layer {
                    dir = Direction::Right;
                }
            }
        }
    }
}

fn maybe_sum(sum: i32, item: &Option<&i32>) -> i32 {
    match item {
        Some(v) => sum + *v,
        None => sum
    }
}

fn sum_surrounds(squares: &HashMap<String, i32>, x: i32, y: i32) -> i32 {
    vec![
        squares.get(&format!("{}|{}", x, y + 1)),
        squares.get(&format!("{}|{}", x - 1, y + 1)),
        squares.get(&format!("{}|{}", x - 1, y)),
        squares.get(&format!("{}|{}", x - 1, y - 1)),
        squares.get(&format!("{}|{}", x, y - 1)),
        squares.get(&format!("{}|{}", x + 1, y - 1)),
        squares.get(&format!("{}|{}", x + 1, y)),
        squares.get(&format!("{}|{}", x + 1, y + 1)),
    ].iter().fold(0, maybe_sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sum_some() {
        let mut m = HashMap::new();
        m.insert("0|0".to_string(), 1);
        m.insert("1|0".to_string(), 1);
        m.insert("1|1".to_string(), 2);
        m.insert("0|1".to_string(), 4);
        m.insert("-1|1".to_string(), 5);

        assert_eq!(sum_surrounds(&m, -1, 0), 10);
    }
}