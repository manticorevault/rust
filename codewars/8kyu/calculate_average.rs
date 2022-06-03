fn find_average(slice: &[f64]) -> f64 {
    let total: f64 = slice.iter().sum();
    if slice.len() > 0 {
        return total / slice.len() as f64;
    }
    0.0
}

// Controlling type is important to perform arithmetic operations