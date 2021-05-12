use std::env;

pub fn parse_argument() -> String {
    let arguments: Vec<String> = env::args().collect();
    let mut parsed_argument: String = arguments[0].clone();

    // When writing formula without whitespace
    // eg: kalko 2+3/2
    if arguments.len() == 2 {
        parsed_argument = arguments[1].clone();
    }
    // When writing formula with whitespace
    // eg: kalko 2 + 3 / 2
    else if arguments.len() > 2 {
        parsed_argument.clear();
        // Concate all arguments into String object
        for (count, string) in (arguments.iter()).enumerate() {
            // Skip first argument since it's the command
            // to execute program
            if count != 0 {
                parsed_argument += string;
            }
        }
    }

    parsed_argument
}
