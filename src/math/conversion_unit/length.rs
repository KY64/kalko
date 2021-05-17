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
