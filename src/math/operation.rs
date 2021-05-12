pub fn multiply(index: usize, value: &mut Vec<f32>, operator: &mut Vec<&str>) {
    // Prevent out of bound when operator and value
    // has the same length
    let i: usize = match value.get(index+1) {
        Some(_) => index,
        _ => index-1
    };

    // Replace current index value with result of operation
    value[i] *= value[i + 1];
    // Remove used number
    value.remove(i + 1);
    // Remove used operator
    operator.remove(index);
}

pub fn divide(index: usize, value: &mut Vec<f32>, operator: &mut Vec<&str>) {
    // Prevent out of bound when operator and value
    // has the same length
    let i: usize = match value.get(index+1) {
        Some(_) => index,
        _ => index-1
    };

    // Replace current index value with result of operation
    value[i] /= value[i + 1];
    // Remove used number
    value.remove(i + 1);
    // Remove used operator
    operator.remove(index);
}

pub fn sum(index: usize, value: &mut Vec<f32>, operator: &mut Vec<&str>) {
    // Prevent out of bound when operator and value
    // has the same length
    let i: usize = match value.get(index+1) {
        Some(_) => index,
        _ => index-1
    };

    // Replace current index value with result of operation
    value[i] += value[i + 1];
    // Remove used number
    value.remove(i + 1);
    // Remove used operator
    operator.remove(index);
}

pub fn substract(index: usize, value: &mut Vec<f32>, operator: &mut Vec<&str>) {
    // Prevent out of bound when operator and value
    // has the same length
    let i: usize = match value.get(index+1) {
        Some(_) => index,
        _ => index-1
    };

    // Replace current index value with result of operation
    value[i] -= value[i + 1];
    // Remove used number
    value.remove(i + 1);
    // Remove used operator
    operator.remove(index);
}
