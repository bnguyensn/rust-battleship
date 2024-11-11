pub fn pluralize(word: &str, count: usize) -> String {
    if count > 1 {
        format!("{}s", word)
    } else {
        word.to_string()
    }
}
