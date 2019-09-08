pub fn sum_num(data: &serde_json::Value) -> i64 {
    let mut sum = 0;

    if data.is_object() {
        let obj = data.as_object().unwrap();

        for (_, value) in obj {
            if value.is_i64() {
                sum = sum + value.as_i64().unwrap();
            } else {
                sum = sum + sum_num(value);
            }
        }
    }

    if data.is_array() {
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
    fn empty_array() {
        let data = json!([]);

        assert_eq!(sum_num(&data), 0);
    }

    #[test]
    fn empty_object() {
        let data = json!({});

        assert_eq!(sum_num(&data), 0);
    }

    #[test]
    fn obj_arr() {
        let data = json!({
            "a":[-1,1]
        });

        assert_eq!(sum_num(&data), 0);
    }

    #[test]
    fn num_obj() {
        let data = json!([
            -1,
            {"a":1}
        ]);

        assert_eq!(sum_num(&data), 0);
    }

    #[test]
    fn nested_arr() {
        let data = json!([
            [
                [3]
            ]
        ]);

        assert_eq!(sum_num(&data), 3);
    }

    #[test]
    fn nested_obj_num() {
        let data = json!({
            "a":{
                "b": 4
            },
            "c": -1
        });

        assert_eq!(sum_num(&data), 3);
    }

    #[test]
    fn arr() {
        let data = json!([1,2,3]);

        assert_eq!(sum_num(&data), 6);
    }

    #[test]
    fn obj() {
        let data = json!({
            "a": 2,
            "b": 4
        });

        assert_eq!(sum_num(&data), 6);
    }
}
