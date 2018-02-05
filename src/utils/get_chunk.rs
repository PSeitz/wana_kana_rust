
///Returns a substring based on start/end values
///
///@param  {String} text
///
///@param  {Number} start index
///
///@param  {Number} end index
///

pub fn get_chunk(text: &str, start: usize, end: usize) -> &str {
    // &text[start..end]
    // text.chars().skip(start).take(end - start).collect()

    let start = text.char_indices().nth(start).map(|el| el.0).unwrap_or(0);
    let end = text.char_indices()
        .nth(end)
        .map(|el| el.0)
        .unwrap_or(text.len());
    &text[start..end]
    // let charso = text.chars().skip(start) as Chars;
    // let charso = charso.take(end - start) as Chars;
    // charso.as_str().to_string()
    // let mut iter = text.chars();
    // iter.skip(start).take(end);
    // iter.by_ref().skip(start).take(end).as_str()
}


///Returns a substring based on start/end values
///
///@param  {String} text
///
///@param  {Number} start index
///
///@param  {Number} end index
///

pub fn get_chunk_chars(text: &Vec<char>, start: usize, end: usize) -> &[char] {
    &text[start..end]
    // text.chars().skip(start).take(end).collect()
    // let mut iter = text.chars();
    // iter.skip(start).take(end);
    // iter.by_ref().skip(start).take(end).as_str()
}
