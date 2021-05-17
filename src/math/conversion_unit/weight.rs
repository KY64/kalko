use crate::math::conversion_unit::{create_unit, relative_unit, Category, PowerBy, Unit, UnitMeasurement};
use Category::*;
use UnitMeasurement::*;


// NOTE:
// All unit is converted from Kilo-xxxx
// eg. 1 Kilometer to 1 Miles = 0.6213712
//     1 Miles to 1 Kilometer = 1.609344
// so use the relative_unit function as follows
// relative_unit(UnitMeasurement, conversion_result)

pub fn gram(string: &str) -> Option<Unit> {
    create_unit(
        string,
        Gram,
        Mass,
        "g",
        "gram",
        vec![
            relative_unit(Gram, 1.0, PowerBy::Ten as i32),
            relative_unit(Pound, 2.2046226, PowerBy::Ten as i32),
            relative_unit(Ounce, 35.27396, PowerBy::Ten as i32),
        ],
        false,
    )
}

pub fn pound(string: &str) -> Option<Unit> {
    create_unit(
        string,
        Pound,
        Mass,
        "lb",
        "pound",
        vec![
            relative_unit(Pound, 1.0, PowerBy::Ten as i32),
            relative_unit(Gram, 0.45359237, PowerBy::Ten as i32),
            relative_unit(Ounce, 16.0, PowerBy::Ten as i32),
        ],
        true,
    )
}


pub fn ounce(string: &str) -> Option<Unit> {
    println!("is feet");
    create_unit(
        string,
        Ounce,
        Mass,
        "oz",
        "ounce",
        vec![
            relative_unit(Ounce, 1.0, PowerBy::Ten as i32),
            relative_unit(Gram, 0.028349523125, PowerBy::Ten as i32),
            relative_unit(Pound, 0.0625, PowerBy::Ten as i32),
        ],
        true,
    )
}
