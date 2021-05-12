use kalko::*;

fn main() {
    // Parse command arguments from terminal
    let arguments: String = parse::parse_argument();
    let (value, operator) = parse_string(&arguments);
    println!("Result: {}", calculate(value, operator));
}
