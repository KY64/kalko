use kalko::*;

fn evaluate(problems: [(&str, f32); 8]) {
    let mut count = 0;
    while count < problems.len() {
        let (value, operator) = parse_string(problems[count].0);
        assert_eq!(calculate(value, operator), problems[count].1);
        count += 1;
    }
}

#[test]
fn basic_operation() {
    const PROBLEMS: [(&str, f32); 8] = [
        ("1+1", 2.0),
        ("1-4", -3.0),
        ("4x19", 76.0),
        ("1/1", 1.0),
        ("10+2-6", 6.0),
        ("6/3x2", 4.0),
        ("2-2x4+3/2", -4.5),
        ("10/2+3-1x10", -2.0),
    ];

    evaluate(PROBLEMS);
}

#[test]
fn negative_value() {
    const PROBLEMS: [(&str, f32); 8] = [
        ("1+-1", 0.0),
        ("1--4", 5.0),
        ("-4x19", -76.0),
        ("-1/-1", 1.0),
        ("10+2--6", 18.0),
        ("-6/-3x-2", -4.0),
        ("2-2x4+-3/2", -7.5),
        ("10/2+3-1x-10", 18.0),
    ];

    evaluate(PROBLEMS);
}

#[test]
fn parentheses() {
    const PROBLEMS: [(&str, f32); 8] = [
        ("1+(-1+2)", 2.0),
        ("(2-1)-4", -3.0),
        ("4x(19+3)", 88.0),
        ("1/(1x2)", 0.5),
        ("10x(2--6)", 80.0),
        ("(-2-2)x4+3/2", -14.5),
        ("-(-2-2)x4+3/2", 17.5),
        ("10/(2+3-1)x10", 25.0),
    ];

    evaluate(PROBLEMS);
}
