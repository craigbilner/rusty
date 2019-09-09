use std::env;
use look_and_say;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        panic!("Please input the input and number of iterations");
    }

    let input = &args[1];

    let iterations: u32 = args[2].trim().parse()
        .expect("Please type a number");

    let result = look_and_say::length(input, iterations);

    println!("The length for {} with {} iterations is {}", input, iterations, result)
}