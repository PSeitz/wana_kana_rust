

/**
 * Tests a character and an english vowel. Returns true if the char is a vowel.
 * @param  {String} char
 * @param  {Boolean} [includeY=true] Optional parameter to include y as a vowel in test
 * @return {Boolean}
 */
pub fn is_char_vowel(char: char, includey = true) -> bool {
  
  const regexp = includeY ? /[aeiouy]/ : /[aeiou]/;
  return char.to_lower_case().chars().nth(0).unwrap().search(regexp) !== -1;
}


