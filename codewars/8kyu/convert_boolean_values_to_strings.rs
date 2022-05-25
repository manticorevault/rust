fn bool_to_word(value: bool) -> &'static str {
    if (value == true) {
        return "Yes"
    } else {
        return "No"
    }
}

// Very good practice 
// fn bool_to_word(value: bool) -> &'static str {
//     match value {
//         true => "Yes",
//         false => "No",
//     }
// }