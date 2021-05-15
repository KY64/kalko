use std::env;
use regex::Regex;
use crate::parse::number::check_negative;

pub fn parse_argument(arg: env::Args) -> String {
    let arguments: Vec<String> = arg.collect();
    if let Some(_) = arguments.iter().position(|x| x.contains("*")) {
        eprintln!("Problem parsing argument: try replace '*' with 'x' for multiplication");
        std::process::exit(1);
    }
    let mut parsed_argument: String = arguments[0].clone();

    // When writing formula without whitespace
    // eg: kalko 2+3/2
    if arguments.len() == 2 {
        parsed_argument = arguments[1].clone();
    }
    // When writing formula with whitespace
    // eg: kalko 2 + 3 / 2
    else if arguments.len() > 2 {
        // Empty string before concatenate with the new String object
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

pub fn parse_math_operation(string: &str) -> Result<(Vec<f32>, Vec<&str>), String> {
    // Find operator +, -, x, /, and parentheses '(', ')'
    // NOTE: Does not support '*' symbol for multiplication since
    // it means "all directory" when used as argument
    let operator_regex = Regex::new(r"[+\-x/()]").unwrap();
    // Find number
    let number_regex = Regex::new(r"\d+").unwrap();
    // Vector of detected number
    let mut value: Vec<f32> = Vec::new();

    // Convert number from &str to f32 then
    // push converted number to Vector
    for i in operator_regex.split(string) {
        // Skip empty value on some case
        if i.is_empty() {
            continue;
        }

        let number = i.parse::<f32>().unwrap_or_else(|err| {
            eprintln!("Problem parsing argument: {}", err);
            std::process::exit(1);
        });
        value.push(number);
    }

    if value.is_empty() {
        return Err(String::from("Value vector is empty!"));
    } 

    // Vector of detected operator
    let mut operator = number_regex.split(string).collect::<Vec<&str>>();

    if operator.is_empty() {
            return Err(String::from("Operator vector is empty!"));
        }

    // Detect whether number is negative
    check_negative(&mut value, &mut operator);

    // Ignore and remove first negative operator as its
    // indicating the first value is negative
    if operator[0] == "-" {
        operator.remove(0);
    }

    Ok((value, operator))
}
