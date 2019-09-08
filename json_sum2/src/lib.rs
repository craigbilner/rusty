pub fn sum_num(data: &serde_json::Value) -> i64 {
    let mut sum = 0;

    if data.is_object() {
        let obj = data.as_object().unwrap();
        let mut has_red = false;

        for (_, value) in obj {
            if value == "red" {
                has_red = true;
                break;
            }
        }

        if has_red {
            return sum;
        }

        for (_, value) in obj {
            if value.is_i64() {
                sum = sum + value.as_i64().unwrap();
            } else {
                sum = sum + sum_num(value);
            }
        }
    } else if data.is_array() {
        let arr = data.as_array().unwrap();

        for value in arr {
            if value.is_i64() {
                sum = sum + value.as_i64().unwrap();
            } else {
                sum = sum + sum_num(value);
            }
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn array_num() {
        let data = json!([1, 2, 3]);

        assert_eq!(sum_num(&data), 6);
    }

    #[test]
    fn red_obj() {
        let data = json!([
            1,
            {
                "c": "red",
                "b": 2
            },
            3
        ]);

        assert_eq!(sum_num(&data), 4);
    }

    #[test]
    fn red_obj_whole() {
        let data = json!({
            "d": "red",
            "e": [1, 2, 3, 4],
            "f": 5
        });

        assert_eq!(sum_num(&data), 0);
    }

    #[test]
    fn red_arr() {
        let data = json!([
            1,
            "red",
            5
        ]);

        assert_eq!(sum_num(&data), 6);
    }
}
