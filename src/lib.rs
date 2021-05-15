mod math;
pub mod parse;

use math::operation::*;
use parse::check_negative;
use regex::Regex;

pub fn parse_string(string: &str) -> (Vec<f32>, Vec<&str>) {
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

    // Vector of detected operator
    let mut operator = number_regex.split(string).collect::<Vec<&str>>();

    // Detect whether number is negative
    check_negative(&mut value, &mut operator).unwrap_or_else(|err| {
        eprintln!("{}", err);
        std::process::exit(1);
    });


    // Ignore and remove first negative operator as its
    // indicating the first value is negative
    if operator[0] == "-" {
        operator.remove(0);
    }


    (value, operator)
}

pub fn calculate(value: Vec<f32>, operator: Vec<&str>) -> f32 {
    let mut clone_value: Vec<f32> = value.clone();
    let mut clone_operator: Vec<&str> = operator.clone();


    // TODO: Need to support parentheses '()' to include in high priority
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

            if _end == _start + 1 {
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
                if operator.len() == clone_operator.len() || clone_operator[_start] == "(" {
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
                if (_start == 0 && _iteration == 1) || clone_operator[_start] == "(" {
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

                if _iteration == 8 {
                    break;
                }
            }
        }
    }

    // Clear empty string to speed up iteration
    clone_operator = clone_operator
        .iter()
        .filter(|x| !x.is_empty())
        .cloned()
        .collect();

    // Start iteration to evaluate number
    iterate_number(&mut clone_value, &mut clone_operator).unwrap_or_else(|err| {
        eprintln!("{}", err);
        std::process::exit(1);
    })
}

fn iterate_number(value: &mut Vec<f32>, operator: &mut Vec<&str>) -> Result<f32, String> {
    let mut index = 0;

    if let Some(x) = operator.first() {
        if *x == "(" {
            operator.remove(0);
        }
    }

    if let Some(x) = operator.last() {
        if *x == ")" {
            operator.remove(operator.len() - 1);
        }
    }

    let clone_operator = operator.clone();
    let mut iteration = clone_operator.iter();


    // Iterate operator while it is not empty
    // to prevent out of bound error
    while !operator.is_empty() {
        let symbol = match iteration.next() {
            Some(x) => x,
            _ => break,
        };

        if operator.iter().position(|x| x.contains("x")).is_some()
            || operator.iter().position(|x| x.contains("/")).is_some()
        {
            // Get operator from vector then unwrap
            // to get the value
            match *symbol {
                "x" => {
                    multiply(index, value, operator);
                    // Run iterate_number again to check whether there is any
                    // operator left
                    iterate_number(value, operator).expect("Operation incomplete!");
                }
                "/" => {
                    divide(index, value, operator);
                    iterate_number(value, operator).expect("Operation incomplete!");
                }
                _ => {
                    index += 1;
                    continue;
                } // Skip the rest part to make sure
                  // the * and / operator are evaluated first
            };
        } else if operator.iter().position(|x| x.contains("+")).is_some()
            || operator.iter().position(|x| x.contains("-")).is_some()
        {
            match *symbol {
                "+" => {
                    sum(index, value, operator);
                    iterate_number(value, operator).expect("Operation incomplete!");
                }
                "-" => {
                    substract(index, value, operator);
                    iterate_number(value, operator).expect("Operation incomplete!");
                }
                _ => {
                    index += 1;
                    continue;
                }
            };
        } else {
            break;
        }
    }
    // Return the result of operation
    if value.len() == 1 {
        Ok(value[0])
    } else {
        Err(format!("Operation incomplete! {:?}", value))
    }
}
