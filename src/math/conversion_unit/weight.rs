use crate::math::conversion_unit::{
    create_unit, relative_unit, Category, PowerBy, Unit, UnitMeasurement,
};
use Category::*;
use UnitMeasurement::*;

// NOTE:
// 7 parameters of create_unit function are
//
// 1. source input
// 2. base unit measurement from UnitMeasurement enum
// 3. category of the unit from Category enum
// 4. abbreviation of the unit
// 5. name of the unit
// 6. the supported unit conversion
// 7. whether it is single unit, means there are no prefix
//    such as kilo, centi, deca
//    eg. Mile, Pound
//
//    if it is single unit, then automatically has Kilo
//    as prefix by default

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
    // relative_unit is a function to create struct
    // of RelativeUnit which has property needed
    // to help the conversion process
    //
    // relative_unit consist of 3 parameters
    //
    // 1. base unit
    // 2. conversion value
    // 3. step
    //    eg. Kilometer to Hectometer is multiply by 10
    //        so every step is powered by 10
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
