/// Returns a substring based on character position start/end values
pub fn get_chunk(text: &str, start: usize, end: usize) -> &str {
    let start = text.char_indices().nth(start).map(|el| el.0).unwrap_or(0);
    let end = text
        .char_indices()
        .nth(end)
        .map(|el| el.0)
        .unwrap_or_else(|| text.len());
    &text[start..end]
}

#[test]
fn get_chunk_test() {
    assert_eq!(get_chunk("derpalerp", 3, 6), "pal");
    assert_eq!(get_chunk("de", 0, 1), "d");
    assert_eq!(get_chunk("", 1, 2), "");
}
