
pub fn search_str<T: AsRef<str>, Q: AsRef<str>>(text: &T, query: &Q) -> bool {
    let text = text.as_ref().to_lowercase();
    let query = query.as_ref().to_lowercase();

    text.contains(&query)
}