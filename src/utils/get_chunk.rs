/// Returns a substring based on character position start/end values
pub fn get_chunk(text: &str, start: usize, end: usize) -> &str {
    let start = text.char_indices().nth(start).map(|el| el.0).unwrap_or(0);
    let end = text.char_indices().nth(end).map(|el| el.0).unwrap_or(text.len());
    &text[start..end]
}
