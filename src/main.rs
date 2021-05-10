use kalko::*;
use std::env;

fn main() {
    // Parse command arguments from terminal
    let arguments: Vec<String> = env::args().collect();
    let mut parse_argument: String = arguments[0].clone();

    // When writing formula without whitespace
    // eg: kalko 2+3/2
    if arguments.len() == 2 {
        parse_argument = arguments[1].clone();
    }
    // When writing formula with whitespace
    // eg: kalko 2 + 3 / 2
    else if arguments.len() > 2 {
        parse_argument.clear();
        // Concate all arguments into String object
        for (count, string) in (arguments.iter()).enumerate() {
            // Skip first argument since it's the command
            // to execute program
            if count != 0 {
                parse_argument += string;
            }
        }
    }

    let (mut value, mut operator) = parse_string(&parse_argument);
    println!("Result: {}", calculate(&mut value, &mut operator));
}
