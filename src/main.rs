use kalko::*;
use std::env;

fn main() {
    // Parse command arguments from terminal
    let arguments: Vec<String> = env::args().collect();
    let (mut value, mut operator) = parse_string(&arguments[1]);

    println!("final result: {}", calculate(&mut value, &mut operator));

}
