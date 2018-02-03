/**
 * Returns a substring based on start/end values
 * @param  {String} text
 * @param  {Number} start index
 * @param  {Number} end index
 * @return {String} new substring
 */
pub fn get_chunk(text: &str, start: usize, end: usize) -> &str {
    &text[start..end]
}
