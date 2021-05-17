use crate::math::conversion_unit::{create_unit, relative_unit, Category, PowerBy, Unit, UnitMeasurement};
use Category::*;
use UnitMeasurement::*;


// NOTE:
// All unit is converted from Kilo-xxxx
// eg. 1 Kilometer to 1 Miles = 0.6213712
//     1 Miles to 1 Kilometer = 1.609344
// so use the relative_unit function as follows
// relative_unit(UnitMeasurement, conversion_result)

pub fn meter(string: &str) -> Option<Unit> {
    create_unit(
        string,
        Meter,
        Length,
        "m",
        "meter",
        vec![
            relative_unit(Meter, 1.0, PowerBy::Ten as i32),
            relative_unit(Miles, 0.6213712, PowerBy::Ten as i32),
            relative_unit(Feet, 3.280839, PowerBy::Ten as i32),
        ],
        false,
    )
}

pub fn mile(string: &str) -> Option<Unit> {
    create_unit(
        string,
        Miles,
        Length,
        "mi",
        "mile",
        vec![
            relative_unit(Miles, 1.0, PowerBy::Ten as i32),
            relative_unit(Meter, 1.609344, PowerBy::Ten as i32),
            relative_unit(Feet, 5.28, PowerBy::Ten as i32),
        ],
        true,
    )
}


pub fn feet(string: &str) -> Option<Unit> {
    println!("is feet");
    create_unit(
        string,
        Feet,
        Length,
        "ft",
        "feet",
        vec![
            relative_unit(Feet, 1.0, PowerBy::Ten as i32),
            relative_unit(Meter, 0.3048, PowerBy::Ten as i32),
            relative_unit(Miles, 0.1893939394, PowerBy::Ten as i32),
        ],
        false,
    )
}
