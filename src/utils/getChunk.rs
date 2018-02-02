/**
 * Returns a substring based on start/end values
 * @param  {String} text
 * @param  {Number} start index
 * @param  {Number} end index
 * @return {String} new substring
 */
fn getChunk(text = '', start = 0, end) {
  return text.slice(start, end);
}

export default getChunk;
