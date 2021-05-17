mod math;
pub mod parse;

use math::operation::iterate_number;
use math::conversion::convert;
use parse::argument::parse_math_operation;
use parse::string::{
    is_conversion, is_math_operation, is_measurement, is_number, parsing, Kind,
};

pub enum Operation {
    Calculation,
    Conversion,
}

//TODO: Refactor code

pub fn parse_string(string: &str) -> Operation {
    // Find operator +, -, x, /, and parentheses '(', ')'
    // NOTE: Does not support '*' symbol for multiplication since
    // it means "all directory" when used as argument
    if is_math_operation(string) {
        Operation::Calculation
    } else if is_conversion(string) {
        Operation::Conversion
    } else {
        eprintln!("Unrecognized argument. Try run kalko --help");
        std::process::exit(1);
    }
}

pub fn conversion(string: &str) -> f32 {
    let mut argument: Vec<&str> = string.split(" ").collect();
    // Remove empty value
    argument = argument.iter().filter(|x| !x.is_empty()).cloned().collect();
    let mut amount: f32;
    let from: &str;
    let to: &str;

    if is_number(argument[0]) {
        // Convert &str to f32
        amount = argument[0].parse().unwrap();
        from = argument[1];
        to = argument[2];
    } else if is_measurement(argument[0]) {
        // Capture number from string
        // eg. "2kg" -> "2"
        //     "29km3" -> "29"
        let number = parsing(Kind::Number)
            .captures(argument[0])
            .unwrap()
            .get(0)
            .unwrap()
            .as_str();
        
        // Convert &str to f32
        amount = number.parse().unwrap();
        // Capture unit from string
        // eg. "2kg" -> "kg"
        //     "29km3" -> "km3"
        from = parsing(Kind::Unit)
            .captures(argument[0])
            .unwrap()
            .get(0)
            .unwrap()
            .as_str();
        to = argument[1];
    } else {
        eprintln!("Unrecognized format");
        std::process::exit(1);
    }

    let result = convert(&mut amount, from, to);
    return result;
}

pub fn calculate(string: &str) -> f32 {
    let (value, operator) = parse_math_operation(string).unwrap_or_else(|err| {
        eprintln!("{}", err);
        std::process::exit(1);
    });
    let mut clone_value: Vec<f32> = value.clone();
    let mut clone_operator: Vec<&str> = operator.clone();

    if operator.iter().position(|x| x.contains("(")).is_some()
        && operator.iter().position(|x| x.contains(")")).is_some()
    {
        // Initialize index
        let mut _start: usize = 0;
        let mut _end: usize = 1;
        // To iterate loop in parentheses and indicate
        // how many parentheses are there
        let mut _iteration = 1;
        // There is a possibility 'start' ,'end' and 'iteration' variable are unused
        // but it's always be used for the first loop iteration, so to avoid
        // compiler warning an underscore has given before them

        loop {
            // Get index position of parentheses
            if let Some(index) = clone_operator.iter().position(|x| x.contains("(")) {
                _start = index;
            } else {
                break;
            }

            if let Some(index) = clone_operator.iter().position(|x| x.contains(")")) {
                _end = index;
            } else {
                break;
            }

            // Prevent removing parentheses if there is
            // multiple operator
            // eg. ["(", ")/", ""] or ["/(", ")", ""]
            // Those will be detected as empty parentheses
            // but because there is '/' symbol, it will not be
            // removed
            if _end == _start + 1 && clone_operator[_start] == "(" && clone_operator[_end] == ")" {
                clone_operator.drain(_start.._end + 1);
                continue;
            }

            // Get only value and operator in parentheses
            //
            // It is not using '.drain' to prevent
            // changing the original vector as it will be
            // replaced with the new value
            let mut truncated_value: Vec<f32> = clone_value[_start.._end].to_vec();
            // If there is nested parentheses, make sure that
            // the first operator is not start with '(' so the
            // truncated value is align with operator
            if operator.len() > clone_operator.len() && !clone_operator[0].starts_with("(") {
                truncated_value = clone_value[_start + 1.._end + 1].to_vec();
            }
            let mut truncated_operator: Vec<&str> = clone_operator[_start + 1.._end].to_vec();

            // In case there are nested parentheses
            // eg. (2+(3-2))
            // update truncated vector and start-end index
            loop {
                if let Some(start_position) =
                    truncated_operator.iter().position(|x| x.contains("("))
                {
                    if let Some(end_position) =
                        truncated_operator.iter().position(|x| x.contains(")"))
                    {
                        _end = end_position + 1;
                    }
                    // start_position + 1 because the truncated operator
                    // index is not the same as clone_operator
                    // index 0 on truncated operator could be
                    // index 1 on clone_operator
                    // eg. clone_operator ["(","+(","-",")-",")"] The '(' detected on 0
                    //     truncated_operator ["+(","-"] The '(' detected on 0
                    // Notice the position of '(' inside truncated_operator is on
                    // index 1 inside the clone_operator, hence
                    // _start = start_position+1
                    _start = start_position + 1;

                    // _start+1 to skip parentheses inside truncated operator
                    truncated_operator = clone_operator.clone()[_start + 1.._end].to_vec();
                    truncated_value = clone_value.clone()[_start.._end].to_vec();

                    break;
                } else {
                    break;
                }
            }

            if let Ok(mut result) = iterate_number(&mut truncated_value, &mut truncated_operator) {
                // If the first operator is "-("
                // convert the result to negative
                if _start == 0 && clone_operator[_start].contains("(") {
                    if clone_operator[_start].starts_with("-") {
                        result = -result;
                    }
                }

                // Replace the evaluated value with result
                // eg. 2+(7+3)x2
                // 7 and 3 will be evaluated first and then
                // the result will be 10. The result then
                // will be placed on the index of 7 and
                // the rest number inside parentheses will
                // be removed.So the result is 2+10x2
                if operator.len() == clone_operator.len() || clone_operator[_start].starts_with("(")
                {
                    clone_value.splice(_start.._end, [result].iter().cloned());
                } else {
                    clone_value.splice(_start + 1.._end + 1, [result].iter().cloned());
                }

                // Trim "(" if the parsed operator is "x(", "+(", etc.
                if clone_operator[_start].len() > 1 {
                    // In case the operator is "x((" it will be trimmed to "x("
                    // then continue iteration in parentheses
                    let length = clone_operator[_start].len() - 1;
                    clone_operator[_start] = &clone_operator[_start][..length];
                }

                // Trim ")" if the parsed operator is ")x", ")-", etc.
                if clone_operator[_end].len() > 1 {
                    clone_operator[_end] = &clone_operator[_end][1..];
                }

                // Iteration here to indicate whether the 0 index is
                // the first parentheses because every time the value inside
                // parentheses has been evaluated, it will be removed
                // so the 0 index operator doesn't necessarily means the
                // first parentheses
                if (_start == 0 && _iteration == 1)
                    || (clone_operator[_start] == "(" && !clone_operator[_end].contains(")"))
                {
                    clone_operator.drain(_start.._end);
                } else {
                    // Start+1 means it will ignore "(" position
                    // because the value inside it is on the next
                    // index
                    // eg. 2+(3+7)x2
                    // Notice that the first operator would be "+("
                    // but the value inside it is on the 2nd index
                    clone_operator.drain(_start + 1.._end);
                }
                _iteration += 1;
            }
        }
    }

    // Trim trailing parentheses
    // eg. ((((1+2))))-2
    // will get result [")))-"] as the last operator. While the value vector is
    // [3.0, -2] so the operation should be evaluated to 3.0 - 2
    // therefore additional trim is needed
    if operator.iter().position(|x| x.contains("(")).is_some()
        || operator.iter().position(|x| x.contains(")")).is_some()
    {
        let parentheses_regex = parsing(Kind::Parentheses);
        // Remove all dangling parentheses
        // eg. [")))-", "x", "+"] becomes ["-", "x", "+"]
        clone_operator = clone_operator
            .iter()
            .map(|op| {
                if op.contains("(") || op.contains(")") {
                    let mut tmp: Vec<&str> = parentheses_regex.split(*op).collect();
                    // Clear empty value after splitting
                    tmp = tmp.iter().filter(|y| !y.is_empty()).cloned().collect();
                    // In case there's only 1 '(' or ')' where the
                    // result would be empty vector, it will return
                    // the original value
                    if tmp.is_empty() {
                        *op
                    } else {
                        // Return filtered operator
                        tmp[0]
                    }
                } else {
                    // Return original value if there is no parentheses
                    *op
                }
            })
            .collect();
    }

    // Clear empty string to speed up iteration
    clone_operator = clone_operator
        .iter()
        .filter(|x| !x.is_empty())
        .cloned()
        .collect();

    // Start iteration to evaluate number
    let result: f32 = iterate_number(&mut clone_value, &mut clone_operator).unwrap_or_else(|err| {
        eprintln!("{}", err);
        std::process::exit(1);
    });

    println!("Result: {}", result);
    result
}
