pub fn check_negative<'a>(
    value: &'a mut Vec<f32>,
    operator: &'a mut Vec<&str>,
) {

    // Detect whether first number is negative
    // it will keep iterating until "-(((-"
    let mut index = 0;
    loop {
        // Prevent out of bound on value vector
        if value.get(index).is_none() {
            break;
        }

        // Ignore if the preceeding operator is ')'
        // eg. 10+(9-2)-3
        // where '3' is not negative number therefore
        // ')-' is ignored
        if operator[index].ends_with("-") && !operator[index].ends_with(")-") {
            if index == 0 || operator[index].len() > 1 {
                value[index] = -value[index];
            }

            if operator[index].len() > 1 {
                operator[index] = &operator[index][..operator[index].len() - 1];
            } else {
                // if operator[index] contains only "-"
                // then increment index
                index += 1;
            }
        } else {
            // if operator[index] not ends with "-"
            // then increment index
            index += 1;
        }
    }
}
