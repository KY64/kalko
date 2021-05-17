pub mod length;
pub mod weight;

// Initialize details of measurement unit
pub struct Unit {
    pub name: String,
    pub alias: String,
    pub base: UnitMeasurement,
    pub prefix: PrefixUnit,
    pub category: Category,
    pub relative: Vec<RelativeUnit>,
}

impl Unit {
    pub fn convert_to(&self, target: &Self) -> Result<f32, &str> {
        for val in self.relative.iter() {
            if val.base == target.base {
                // Count the distance from source unit and target unit
                // for example Kilo to Centi is 5 'level' away, so
                // it can be known by calculate different between
                // target prefix and source prefix with value already
                // defined on enum PrefixUnit
                // Centi = 6
                // Kilo = 1
                // the different is 5, then start convert the range
                // by calculate range * (step ^ 5) ['step' power by 5]
                let power_by = target.prefix as i32 - self.prefix as i32;
                // If converting the same unit, change step to 1
                // eg. 10m m will result 10
                if power_by == 0 {
                    return Ok(convert_range(val.range, val.step, power_by));
                }
                return Ok(convert_range(val.range, val.step, power_by));
            }
        }

        Err("Failed to convert unit")
    }
}

// Added PartialEq trait to support comparison like a == b
// it doesn't use #[derive] directly because not all
// property will be compared
impl PartialEq for Unit {
    fn eq(&self, other: &Self) -> bool {
        // Only compare base and category
        // property
        if self.base == other.base {
            self.base == other.base
        } else {
            self.category == other.category
        }
    }
}

// Enum for step in conversion unit
// eg. cm to mm is multiply by 10
// therefore Ten enum is used
pub enum PowerBy {
    Ten = 10,
    //Hundred = 100, // Commented for further use
    //Thousand = 1000,
}

// PrefixUnit is an indicator to know
// what is the prefix of the detected unit
// and to help conversion process
//
// This enum derive Clone and Copy traits because
// its value is moved when the value is read from
// struct, therefore to avoid error, Clone and Copy
// traits are derived
#[derive(Clone, Copy)]
pub enum PrefixUnit {
    Kilo = 1,
    Hecto,
    Deca,
    None,
    Deci,
    Centi,
    Milli,
}

// Initialize details of conversion unit
pub struct RelativeUnit {
    pub base: UnitMeasurement,
    pub range: f32,
    pub step: i32,
}

// List of current supported measurement unit
// it's deriving PartialEq traits because
// the value will be used to compare between
// 2 units for conversion process
#[derive(PartialEq)]
pub enum UnitMeasurement {
    Meter,
    Gram,
    Miles,
    Feet,
    Ounce,
    Pound,
}

// Category of measurement unit
// it's deriving PartialEq traits because
// the value will be used for evaluation
// whether source and target unit has the
// same category to support conversion
#[derive(PartialEq)]
pub enum Category {
    Length,
    Mass,
}

// Create RelativeUnit struct
pub fn relative_unit(unit: UnitMeasurement, conversion_range: f32, step: i32) -> RelativeUnit {
    RelativeUnit {
        base: unit,
        range: conversion_range,
        step: step,
    }
}

// Change conversion value
fn convert_range(range: f32, step: i32, pow: i32) -> f32 {
    range * (step as f32).powi(pow)
}

// Find prefix of the unit and set PrefixUnit enum
fn find_prefix(
    string: &str,
    alias: &str,
    name: &str,
    single_unit: bool,
) -> (PrefixUnit, String, String) {
    // Single unit means there are no prefix like Kilo, Hecto, Centi, etc.
    // eg. Miles does not have any prefix
    if single_unit {
        return (PrefixUnit::Kilo, name.to_string(), alias.to_string());
    }

    // Dynamically create name and alias for unit
    let names: [(&str, &str); 7] = [
        (&format!("k{}", alias), &format!("kilo{}", name)),
        (&format!("h{}", alias), &format!("hecto{}", name)),
        (&format!("da{}", alias), &format!("deca{}", name)),
        (&format!("{}", alias), &format!("{}", name)),
        (&format!("d{}", alias), &format!("deci{}", name)),
        (&format!("c{}", alias), &format!("centi{}", name)),
        (&format!("m{}", alias), &format!("milli{}", name)),
    ];

    // Return detected prefix
    for val in names.iter() {
        if string == val.0 || string == val.1 {
            if val.1.starts_with("kilo") {
                return (PrefixUnit::Kilo, val.0.to_string(), val.1.to_string());
            } else if val.1.starts_with("hecto") {
                return (PrefixUnit::Hecto, val.0.to_string(), val.1.to_string());
            } else if val.1.starts_with("deca") {
                return (PrefixUnit::Deca, val.0.to_string(), val.1.to_string());
            } else if val.1.starts_with(name) {
                return (PrefixUnit::None, val.0.to_string(), val.1.to_string());
            } else if val.1.starts_with("deci") {
                return (PrefixUnit::Deci, val.0.to_string(), val.1.to_string());
            } else if val.1.starts_with("centi") {
                return (PrefixUnit::Centi, val.0.to_string(), val.1.to_string());
            } else if val.1.starts_with("milli") {
                return (PrefixUnit::Milli, val.0.to_string(), val.1.to_string());
            }
        }
    }

    eprintln!("Prefix not found! It may be not supported for the current version");
    std::process::exit(1);
}

// Create measurement unit
pub fn create_unit(
    string: &str,
    base: UnitMeasurement,
    category: Category,
    alias: &str,
    name: &str,
    conversion_units: Vec<RelativeUnit>,
    single_unit: bool,
) -> Option<Unit> {
    if conversion_units.is_empty() {
        return None;
    }

    let (prefix, unit_alias, unit_name) = find_prefix(string, alias, name, single_unit);

    Some(Unit {
        name: unit_name,
        alias: unit_alias,
        base: base,
        prefix: prefix,
        category: category,
        relative: conversion_units,
    })
}
