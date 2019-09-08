use std::io::{self, Read};
use json_sum;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).expect("Could not read input");

    let data: serde_json::Value = serde_json::from_str(&buffer).unwrap();

    println!("The total is {}", json_sum::sum_num(&data));
}