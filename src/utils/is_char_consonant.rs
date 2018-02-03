

/**
 * Tests a character and an english consonant. Returns true if the char is a consonant.
 * @param  {String} char
 * @param  {Boolean} [includeY=true] Optional parameter to include y as a consonant in test
 * @return {Boolean}
 */
pub fn isCharConsonant(char: char, includeY = true) -> bool {
  
  const regexp = includeY ? /[bcdfghjklmnpqrstvwxyz]/ : /[bcdfghjklmnpqrstvwxz]/;
  return char.toLowerCase().charAt(0).search(regexp) !== -1;
}

export default isCharConsonant;
