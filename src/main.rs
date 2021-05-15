use kalko::*;
use std::env;

fn main() {
    // Parse command arguments from terminal
    let arguments: String = parse::parse_argument(env::args());
    let (value, operator) = parse_string(&arguments);
    println!("Result: {}", calculate(value, operator));
}
