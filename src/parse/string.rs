use regex::Regex;

// Type of parser
pub enum Kind {
    Char,
    Conversion,  // Unit conversion eg. "1m miles", "10 kg gram"
    Math,        // Basic math operation eg. "1+3-(10x3)/3"
    Measurement, // Measurement unit eg. "1m", "2 kg3"
    Number, // To detect number that mixed with other character
    NumberOnly, // To detect number without any other character
    Operator, // Math operator "+", "-", "x", "/"
    Parentheses,
    Unit, // "kg", "mm", "m3"
}

pub fn regex(expression: &str) -> Regex {
    Regex::new(expression).unwrap_or_else(|err| {
        eprintln!("{}", err);
        std::process::exit(1);
    })
}

// Returning Regex to be used as parser
pub fn parsing(parser: Kind) -> Regex {
    match parser {
        Kind::Char => regex(r"[a-zA-Z]+"),
        Kind::Conversion => regex(r"^\d+(\.\d+)?\s?[a-zA-Z]+[23]?\s[a-zA-Z]+[23]?$"),
        Kind::Math => regex(r"(-?\d+[+/x\-](-)*\d+)+|(\(-?\d+[+/x\-](-)*\d+\))+"),
        Kind::Measurement => regex(r"^\d+(\.\d+)?\s?[a-zA-Z]+[23]?$"),
        Kind::Number => regex(r"\d+(\.\d+)?"),
        Kind::NumberOnly => regex(r"^\d+(\.\d+)?$"),
        Kind::Operator => regex(r"[+\-x/()]"),
        Kind::Parentheses => regex(r"[()]"),
        Kind::Unit => regex(r"[a-zA-Z]+[23]?"),
    }
}

// Helper function to match string with regex
fn is_match(parser: Kind, string: &str) -> bool {
    parsing(parser).is_match(string)
}

pub fn is_char(string: &str) -> bool {
    is_match(Kind::Char, string)
}

pub fn is_conversion(string: &str) -> bool {
    is_match(Kind::Conversion, string)
}

pub fn is_math_operation(string: &str) -> bool {
    is_match(Kind::Math, string)
}

pub fn is_measurement(string: &str) -> bool {
    is_match(Kind::Measurement, string)
}

pub fn is_number(string: &str) -> bool {
    is_match(Kind::NumberOnly, string)
}

pub fn is_operator(string: &str) -> bool {
    is_match(Kind::Operator, string)
}

pub fn is_unit(string: &str) -> bool {
    is_match(Kind::Unit, string)
}

pub fn match_unit(string: &str, alias: &str, name: &str) -> bool {
    let expression = &format!(
        r"(^(k|h|da|d|c|m)?{}$|^(kilo|hecto|deca|deci|centi|milli)?{}$)",
        alias, name
    );
    let parse_unit = regex(expression);
    parse_unit.is_match(string)
}
