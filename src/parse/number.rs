pub fn check_negative<'a>(
    value: &'a mut Vec<f32>,
    operator: &'a mut Vec<&str>,
) -> Result<(), String> {
    if value.is_empty() {
        return Err(String::from("Value vector is empty!"));
    } else if operator.is_empty() {
        return Err(String::from("Operator vector is empty!"));
    }

    // Detect whether first number is negative
    // it will keep iterating until "-(((-"
    let mut index = 0;
    loop {
        // Prevent out of bound on value vector
        if value.get(index).is_none() {
            break;
        }

        if operator[index].ends_with("-") && !operator[index].starts_with(")") {
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

    Ok(())
}
