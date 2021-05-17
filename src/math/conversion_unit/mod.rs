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

impl PartialEq for Unit {
    fn eq(&self, other: &Self) -> bool {
        if self.base == other.base {
            self.base == other.base
        } else {
            self.category == other.category
        }
    }
}

pub enum PowerBy {
    Ten = 10,
    Hundred = 100,
    Thousand = 1000,
}


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
#[derive(Clone)]
pub struct RelativeUnit {
    pub base: UnitMeasurement,
    pub range: f32,
    pub step: i32,
}

// List of current supported measurement unit
#[derive(Clone, PartialEq)]
pub enum UnitMeasurement {
    Meter,
    Gram,
    Miles,
    Feet,
    Ounce,
    Pound,
}

// Category of measurement unit
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

fn convert_range(range: f32, step: i32, pow: i32) -> f32 {
    range * (step as f32).powi(pow)
}

// Change range property in RelativeUnit struct
// this is useful to create conversion for example
// from kilo-xxxx to centi-xxxx the range will be divided by 10000
// eg.
//
// let mut kilometer = vec![relative_unit(Miles, 0.6213712)]
// let centimeter = convert_unit(&mut kilometer, 10000)
//
// From the example above, kilometer has conversion unit to Miles
// and the range is 0.6213712. To convert from centimeter, it means
// it needs to be divided by 10000 first, so using convert_unit
// function it can be done by typing:
//
// convert_unit(&mut kilometer, 10000)

pub fn convert_unit(unit: &mut Vec<RelativeUnit>, number: f32) -> Vec<RelativeUnit> {
    for value in unit.iter_mut() {
        value.range /= number;
        println!("range {}", value.range);
    }
    unit.clone()
}

fn find_prefix(string: &str, alias: &str, name: &str, single_unit: bool) -> (PrefixUnit, String, String) {

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
        relative: conversion_units
    })

    /*
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

    // The initialization will start from
    // Kilometer as the conversion unit is
    // base on Kilometer. So it will be
    // divided by 1.0 at the start, then
    // increment by 10 as it's stepping down
    let mut num = 1.0;
    for val in names.iter() {
        println!("name unit {}", val.1);
        if string == val.0 || string == val.1 {
            println!("name unit {}", val.1);
            return Some(Unit {
                name: unit_name,
                alias: unit_alias,
                base: base,
                prefix: prefix,
                category: category,
                relative: convert_unit(&mut conversion_units.clone(), num),
            });
        }
        if single_unit {
            continue;
        }
        num *= 10.0;
    }
    */
}
