/**
 * Returns a substring based on start/end values
 * @param  {String} text
 * @param  {Number} start index
 * @param  {Number} end index
 * @return {String} new substring
 */
pub fn get_chunk(text: &str, start: usize, end: usize) -> String {
    // &text[start..end]
    text.chars().skip(start).take(end - start).collect()
    // let mut iter = text.chars();
    // iter.skip(start).take(end);
    // iter.by_ref().skip(start).take(end).as_str()
}

/**
 * Returns a substring based on start/end values
 * @param  {String} text
 * @param  {Number} start index
 * @param  {Number} end index
 * @return {String} new substring
 */
pub fn get_chunk_chars(text: &Vec<char>, start: usize, end: usize) -> &[char] {
    &text[start..end]
    // text.chars().skip(start).take(end).collect()
    // let mut iter = text.chars();
    // iter.skip(start).take(end);
    // iter.by_ref().skip(start).take(end).as_str()
}
