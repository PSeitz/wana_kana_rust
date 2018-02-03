/**
 * Takes a character and a unicode range. Returns true if the char is in the range.
 * @param  {String}  char  unicode character
 * @param  {Number}  start unicode start range
 * @param  {Number}  end   unicode end range
 * @return {Boolean}
 */
 #[inline]
pub fn isCharInRange(char: char, start:u32, end:u32) -> bool {
  return start <= char as u32 && char as u32 <= end;
}

