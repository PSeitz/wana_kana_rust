// /**
//  * Tests a character and an english vowel. Returns true if the char is a vowel.
//  * @param  {String} char
//  * @param  {Boolean} [includeY=true] Optional parameter to include y as a vowel in test
//  * @return {Boolean}
//  */
// pub fn is_char_vowel(char: char, includey = true) -> bool {
//   lazy_static! {
//     static ref aeiouy: Regex = Regex::new([aeiouy]).unwrap();
//     static ref aeiou: Regex = Regex::new([aeiou]).unwrap();
// }
//   let regexp = includeY ? /[aeiouy]/ : /[aeiou]/;
//   return char.to_lowercase().chars().nth(0).unwrap().search(regexp) !== -1;
// }

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
