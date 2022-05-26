fn expressions_matter(a: u64, b: u64, c: u64) -> u64 {
    *[ // The * in the beginning of the array provides the same type u64 instead of &u64
        // Present the combinations 
        a + b + c,
        a * (b + c),
        (a + b) * c,
        a * b * c
    ]   
    .iter() // Iterate over them 
    .max() // Get the maximum value
    .unwrap() // “Give me the result of the computation, and if there was an error, panic and stop the program.”
}