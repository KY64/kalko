use regex::Regex;

pub fn parse_string(string: &str) -> (Vec<f32>, Vec<&str>) {
    // Find operator +, -, x, /
    // NOTE: Does not support '*' for multiplication since
    // it means "all directory" when used as argument
    let operator_regex = Regex::new(r"[+\-x/]").unwrap();
    // Find number
    // TODO: Need regular expression to detect negative
    //       number
    let number_regex = Regex::new(r"\d+").unwrap();
    // Vector of detected operator
    let mut operator = number_regex.split(string).collect::<Vec<&str>>();
    // Clear empty string
    operator = operator.drain(1..operator.len() - 1).collect();
    // Vector of detected number
    let mut value: Vec<f32> = Vec::new();

    // Convert number from &str to f32 then
    // push converted number to Vector
    for i in operator_regex.split(string) {
        // TODO: Should not panic when error occured, it needs to
        //       exit cleanly or add clear error message
        let number = i.parse::<f32>().expect("Failed to parse arguments");
        value.push(number);
    }

    (value, operator)
}

pub fn calculate(value: &mut Vec<f32>, operator: &mut Vec<&str>) -> f32 {
    let mut count = 0;
    let clone_operator = operator.clone();
    let mut iteration = clone_operator.iter();

    // Iterate operator while it is not empty
    // to prevent out of bound error
    while !operator.is_empty() {
        if operator.contains(&"x") || operator.contains(&"/") {
            // Get operator from vector then unwrap
            // to get the value
            match iteration.next().unwrap() {
                // Add '&' to convert str to &str
                // since the matched value type is &str
                &"x" => {
                    // Replace current index value with result of operation
                    value[count] *= value[count + 1];
                    // Remove used number
                    value.remove(count + 1);
                    // Remove used operator
                    operator.remove(count);
                    // Run calculate again to check whether there is any
                    // operator left
                    calculate(value, operator);
                }
                &"/" => {
                    value[count] /= value[count + 1];
                    value.remove(count + 1);
                    operator.remove(count);
                    calculate(value, operator);
                }
                _ => {
                    count += 1;
                    continue;
                } // Skip the rest part to make sure
                  // the * and / operator are evaluated first
            };
        } else if operator.contains(&"+") || operator.contains(&"-") {
            match iteration.next().unwrap() {
                &"+" => {
                    value[count] += value[count + 1];
                    value.remove(count + 1);
                    operator.remove(count);
                    calculate(value, operator);
                }
                &"-" => {
                    value[count] -= value[count + 1];
                    value.remove(count + 1);
                    operator.remove(count);
                    calculate(value, operator);
                }
                _ => {
                    count += 1;
                    continue;
                }
            };
        } else {
            break;
        }
    }
    // Return the result of operation
    value[0]
}
