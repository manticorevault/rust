fn name_shuffler(s: &str) -> String {
    s.rsplit(" ") // Splits the s &str in reverse
        .collect::<Vec<&str>>() // Collects it in the <Vec<&str>> type
        .join(" ") // Joins the string again
}