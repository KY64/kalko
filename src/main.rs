use std::env;
use regex::Regex;

fn main() {
    // Parse command arguments from terminal
    let arguments: Vec<String> = env::args().collect();
    // Find operator +, -, *, /
    let basic_operator = Regex::new(r"[+\-*/]").unwrap();
    // Find number
    let number = Regex::new(r"\d+").unwrap();
    // Vector of detected operator
    let mut operator = number.split(&arguments[1]).collect::<Vec<&str>>();
    // Clear empty string
    operator = operator.drain(1..operator.len()-1).collect();
    // Vector of detected number
    let mut value: Vec<f32> = Vec::new();

    // Convert number from &str to f32
    // Push to Vector value
    for i in basic_operator.split(&arguments[1]) {
        value.push(i.parse::<f32>().unwrap());
    }

    println!("final result: {}", calculate(&mut value, &mut operator));

}

fn calculate<'a>(value:&mut Vec<f32>, operator:&'a mut Vec<&str>) -> f32 {
    // Prevent out of bond and return result
    if operator.is_empty() {
        return value[0];
    } else if operator.contains(&"*") || operator.contains(&"/") {
        for count in 0..operator.len() {
            // Prevent out of bond and return result
            if operator.is_empty() {
                return value[0];
            }
            match &operator[count] {
                &"*" => {
                    // Replace current index value with result of operation
                    value[count] *= value[count+1];
                    // Remove used number
                    value.remove(count+1);
                    // Remove used operator
                    operator.remove(count);
                    // Run calculate again to check whether there is any
                    // operator left
                    calculate(value,operator);
                },
                &"/" => {
                    value[count] /= value[count+1];
                    value.remove(count+1);
                    operator.remove(count);
                    calculate(value,operator);
                }
                _ => continue
            };
        }
    } else if operator.contains(&"+") || operator.contains(&"-") {
        for count in 0..operator.len() {
            if operator.is_empty() {
                return value[0];
            }
            match &operator[count] {
                &"+" => {
                    value[count] += value[count+1];
                    value.remove(count+1);
                    operator.remove(count);
                    calculate(value,operator);
                },
                &"-" => {
                    value[count] -= value[count+1];
                    value.remove(count+1);
                    operator.remove(count);
                    calculate(value,operator);
                }
                _ => continue
            };
        }
    }
    value[0]
}
