use kalko::*;

pub fn evaluate(problems: [(&str, f32); 10]) {
    let mut count = 0;
    while count < problems.len() {
        assert_eq!(conversion(problems[count].0), problems[count].1);
        count += 1;
    }
}

#[test]
fn length_conversion() {
    
    const PROBLEMS: [(&str, f32); 10] = [
        ("1m m", 1.0),
        ("10km m", 10000.0),
        ("1 m mi", 0.00062137126),
        ("27mm mi", 0.000016777023),
        ("0.23mm km", 0.00000023),
        ("921ft mi", 0.17443183),
        ("10 kilofeet mile", 1.8939394),
        ("10.9 meter mi", 0.0067729466),
        ("132.990 millimeter dm", 1.3299),
        ("31 mile kilometer", 49.889664),
    ];
    
    evaluate(PROBLEMS);
}

#[test]
fn weight_conversion() {
    
    const PROBLEMS: [(&str, f32); 10] = [
        ("1g g", 1.0),
        ("10kg g", 10000.0),
        ("1 g lb", 0.0022046226),
        ("27mg lb", 0.000059524806),
        ("0.23mg kg", 0.00000023),
        ("921oz lb", 57.5625),
        ("10 lb oz", 160.0),
        ("10.9 gram oz", 0.38448617),
        ("132.990 milligram dg", 1.3299),
        ("31 lb kilogram", 14.061363),
    ];
    
    evaluate(PROBLEMS);
}
