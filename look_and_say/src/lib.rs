pub fn length(val: &str, iterations: u32) -> usize {
    let mut s = String::from(val);

    for _ in 0..iterations {
        s = next(s)
    }

    s.len()
}

fn next(val: String) -> String {
    let mut new = String::new();
    let mut prev = '\0';
    let mut count = 0;

    for num in val.chars() {
        if num == prev {
            count = count + 1;
        } else {
            if count > 0 {
                new = format!("{}{}{}", new, count, prev);
            }

            prev = num;
            count = 1;
        }
    }

    format!("{}{}{}", new, count, prev)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(next("1".to_string()), "11");
    }

    #[test]
    fn test_2() {
        assert_eq!(next("11".to_string()), "21");
    }

    #[test]
    fn test_3() {
        assert_eq!(next("21".to_string()), "1211");
    }

    #[test]
    fn test_4() {
        assert_eq!(next("1211".to_string()), "111221");
    }

    #[test]
    fn test_5() {
        assert_eq!(next("111221".to_string()), "312211");
    }
}
