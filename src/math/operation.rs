pub fn iterate_number(value: &mut Vec<f32>, operator: &mut Vec<&str>) -> Result<f32, String> {
    let mut index = 0;

    // Remove parentheses because it's already
    // processed on calculate function
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

fn multiply(index: usize, value: &mut Vec<f32>, operator: &mut Vec<&str>) {
    // Prevent out of bound when operator and value
    // has the same length
    let i: usize = match value.get(index + 1) {
        Some(_) => index,
        _ => index - 1,
    };

    // Replace current index value with result of operation
    value[i] *= value[i + 1];
    // Remove used number
    value.remove(i + 1);
    // Remove used operator
    operator.remove(index);
}

fn divide(index: usize, value: &mut Vec<f32>, operator: &mut Vec<&str>) {
    // Prevent out of bound when operator and value
    // has the same length
    let i: usize = match value.get(index + 1) {
        Some(_) => index,
        _ => index - 1,
    };

    // Replace current index value with result of operation
    value[i] /= value[i + 1];
    // Remove used number
    value.remove(i + 1);
    // Remove used operator
    operator.remove(index);
}

fn sum(index: usize, value: &mut Vec<f32>, operator: &mut Vec<&str>) {
    // Prevent out of bound when operator and value
    // has the same length
    let i: usize = match value.get(index + 1) {
        Some(_) => index,
        _ => index - 1,
    };

    // Replace current index value with result of operation
    value[i] += value[i + 1];
    // Remove used number
    value.remove(i + 1);
    // Remove used operator
    operator.remove(index);
}

fn substract(index: usize, value: &mut Vec<f32>, operator: &mut Vec<&str>) {
    // Prevent out of bound when operator and value
    // has the same length
    let i: usize = match value.get(index + 1) {
        Some(_) => index,
        _ => index - 1,
    };

    // Replace current index value with result of operation
    value[i] -= value[i + 1];
    // Remove used number
    value.remove(i + 1);
    // Remove used operator
    operator.remove(index);
}
