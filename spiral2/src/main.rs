use std::env;
use spiral2;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("Please type a number");
    }

    let value: u32 = args[1].trim().parse()
        .expect("Please type a number");

    println!("The first number greater than {} is {}", value, spiral2::greater_than(value))
}
