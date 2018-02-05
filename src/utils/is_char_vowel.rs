// /**
//  * Tests a character and an english vowel. Returns true if the char is a vowel.
//  * @param  {String} char
//  * @param  {Boolean} [includeY=true] Optional parameter to include y as a vowel in test
//  * @return {Boolean}
//  */
/**
 * Tests a character and an english vowel. Returns true if the char is a vowel.
 * @param  {String} char
 * @param  {Boolean} [includeY=true] Optional parameter to include y as a vowel in test
 * @return {Boolean}
 */
pub fn is_char_vowel(char: char) -> bool {
    match char {
        'a' | 'e' | 'i' | 'o' | 'u' | 'y' => true,
        _ => false,
    }
}
