pub fn extract_first_char(string: &str) -> (char, &str) {
    let mut chars = string.chars();
    let first_char = chars
        .next()
        .expect("Could not extract character because string is empty");

    (first_char, chars.as_str())
}
