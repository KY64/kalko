use kalko::*;
use std::env;

fn main() {
    // Parse command arguments from terminal
    let arguments: Vec<String> = env::args().collect();
    let mut parse_argument = &arguments[0];

    // temp_string to concate all arguments
    // it's placed here to prevent value
    // being moved before borrowed
    let mut temp_string = String::new(); // TODO: Find replacement for temp_string
                                         //       since it's possible not being used
                                         //       when there are only 2 arguments

    // When writing formula without whitespace
    // eg: kalko 2+3/2
    if arguments.len() == 2 {
        parse_argument = &arguments[1];
    }
    // When writing formula with whitespace
    // eg: kalko 2 + 3 / 2
    else if arguments.len() > 2 {
        // Concate all arguments into String object
        for (count, string) in (arguments.iter()).enumerate() {
            // Skip first argument since it's the command
            // to execute program
            if count != 0 {
                temp_string += string;
            }
        }
        // Convert String object to &str
        parse_argument = &temp_string;
    }

    let (mut value, mut operator) = parse_string(parse_argument);
    println!("Result: {}", calculate(&mut value, &mut operator));
}
