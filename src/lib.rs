mod math;
pub mod parse;

use math::operation::*;
use parse::check_negative;
use regex::Regex;

pub fn parse_string(string: &str) -> (Vec<f32>, Vec<&str>) {
    // Find operator +, -, x, /
    // NOTE: Does not support '*' for multiplication since
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
            println!("Problem parsing argument: {}", err);
            std::process::exit(1);
        });
        value.push(number);
    }

    // Vector of detected operator
    let mut operator = number_regex.split(string).collect::<Vec<&str>>();

    // Detect whether number is negative
    check_negative(&mut value, &mut operator).unwrap_or_else(|err| {
        println!("{}", err);
        std::process::exit(1);
    });

    println!("value {:?}", value);
    println!("operator {:?}\n\n", operator);

    // Ignore and remove first negative operator as its
    // indicating the first value is negative
    if operator[0] == "-" {
        operator.remove(0); 
    }


    println!("trimmed operator {:?}\n\n", operator);

    (value, operator)
}

pub fn calculate(value: Vec<f32>, operator: Vec<&str>) -> f32 {
    let mut clone_value: Vec<f32> = value.clone();
    let mut clone_operator: Vec<&str> = operator.clone();


    println!("current value: {:?}", clone_value);
    println!("current operator: {:?}\n\n", clone_operator);

    // TODO: Need to support parentheses '()' to include in high priority
    // NOTE: The keys are compare the length of value and operator vector,
    //       length of the string, index, and starts/ends with
    if operator.iter().position(|x| x.contains("(")).is_some()
        && operator.iter().position(|x| x.contains(")")).is_some() {
        // Initialize index
       let mut start: usize = 0;
       let mut end: usize = 1;

       loop {
           // Get index position of parentheses
           if let Some(index) = clone_operator.iter().position(|x| x.contains("(")) {
               start = index;
           } else {
               break;
           }

           if let Some(index) = clone_operator.iter().position(|x| x.contains(")")) {
               end = index;
           } else {
               break;
           }
           println!("start {} position {}", clone_operator[start], start);
           println!("end {} position {}", clone_operator[end], end);

           // Get only value and operator in parentheses
           //
           // It is not using '.drain' to prevent
           // changing the original vector as it will be
           // replaced with the new value
           let mut truncated_value: Vec<f32> = clone_value[start..end].to_vec();
           let mut truncated_operator: Vec<&str> = clone_operator[start+1..end].to_vec();

           println!("truncated operator {:?}", truncated_operator);

           if let Ok(mut result) = iterate_number(&mut truncated_value, &mut truncated_operator) {

               if clone_operator[0].contains("(") {
                   if clone_operator[0].starts_with("-") {
                       result = -result;
                   }
               }
               
               clone_value.splice(start..end, [result].iter().cloned());

               if clone_operator[start].len() > 1 {
                   clone_operator[start] = &clone_operator[start][..1];
               }
                
               if clone_operator[end].len() > 1 {
                   clone_operator[end] = &clone_operator[end][1..];
               }

               println!("before {:?}", clone_operator);
               // Start+1 means it will ignore "(" as it is
               // in case the start index is "-(" because
               // it will be converted to negative value
               // in further process
               if start == 0 {
                   clone_operator.drain(start..end);
               } else {
                   clone_operator.drain(start+1..end);
               }
               println!("after {:?}", clone_operator);
               //calculate(clone_value.clone(), clone_operator.clone());
           }
       }

    }

    // Clear empty string to speed up iteration
    clone_operator = clone_operator.iter().filter(|x| !x.is_empty()).cloned().collect();

    iterate_number(&mut clone_value, &mut clone_operator).unwrap_or_else(|msg| {
        println!("{}", msg);
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
            operator.remove(operator.len()-1);
        }
    }

    let clone_operator = operator.clone();
    let mut iteration = clone_operator.iter();

    println!("\n\niteration value {:?}", value);
    println!("iteration operator {:?}", operator);

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
            match symbol {
                // Add '&' to convert str to &str
                // since the matched value type is &str
                &"x" => {
                    multiply(index, value, operator);
                    // Run iterate_number again to check whether there is any
                    // operator left
                    iterate_number(value, operator).expect("Operation incomplete!");
                }
                &"/" => {
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
            match symbol {
                &"+" => {
                    sum(index, value, operator);
                    iterate_number(value, operator).expect("Operation incomplete!");
                }
                &"-" => {
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
