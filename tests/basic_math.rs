use kalko::*;

#[test]
fn basic_math() {
    const PROBLEMS: [(&str, f32); 8] = [
        ("1+1", 2.0), 
        ("1-4", -3.0), 
        ("4*19", 76.0), 
        ("1/1", 1.0), 
        ("10+2-6", 6.0), 
        ("6/3*2", 4.0),
        ("2-2*4+3/2", -4.5), 
        ("10/2+3-1*10", -2.0)
    ];

    let mut count = 0;
    while count < PROBLEMS.len() {
        let (mut value, mut operator) = parse_string(PROBLEMS[count].0);
        assert_eq!(calculate(&mut value, &mut operator), PROBLEMS[count].1);
        count+=1;
    }
}
