use kalko::parse::parse_argument;
use kalko::{calculate, conversion, parse_string, Operation};
use std::env;

fn main() {
    // Parse command arguments from terminal
    let arguments: String = parse_argument(env::args());
    match parse_string(&arguments) {
        Operation::Calculation => calculate(&arguments),
        Operation::Conversion => conversion(&arguments),
    };
}
