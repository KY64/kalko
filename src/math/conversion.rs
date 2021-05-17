use crate::math::conversion_unit::*;
use crate::parse::match_unit;

pub fn convert(amount: &mut f32, from: &str, to: &str) -> f32 {
    if let Some(detected) = detect_unit(from) {
        if let Some(target) = detect_unit(to) {
            if detected.category != target.category {
                eprintln!("Error converting unrelated unit");
                std::process::exit(1);
            }

            *amount *= detected.convert_to(&target).unwrap_or_else(|err| {
                eprintln!("{}", err);
                std::process::exit(1);
            })

        } else {
            eprintln!("{} currently not supported", to);
            std::process::exit(1);
        }
    } else {
        eprintln!("{} currently not supported", from);
        std::process::exit(1);
    }

    println!("Result: {}", amount);
    *amount
}

fn detect_unit(string: &str) -> Option<Unit> {
    if match_unit(string, "m", "meter") {
        length::meter(string)
    } else if match_unit(string, "mi", "mile") {
        length::mile(string)
    } else if match_unit(string, "ft", "feet") {
        length::feet(string)
    } else if match_unit(string, "g", "gram") {
        weight::gram(string)
    } else if match_unit(string, "lb", "pound") {
        weight::pound(string)
    } else if match_unit(string, "oz", "ounce") {
        weight::ounce(string)
    } else {
        None
    }
}
