/// Tests a character and an english vowel. Returns true if the char is a vowel.
pub fn is_char_vowel(char: char) -> bool {
    is_char_vowel_opt(char, true)
}

/// Tests a character and an english vowel. Returns true if the char is a vowel.
///
/// * `include_y` include y as a vowel in test
pub fn is_char_vowel_opt(char: char, include_y: bool) -> bool {
    match char {
        'a' | 'e' | 'i' | 'o' | 'u' => true,
        'y' if include_y => true,
        _ => false,
    }
}

#[test]
fn is_char_vowel_test() {
    assert!(!is_char_vowel_opt('y', false));
    assert!(is_char_vowel_opt('y', true));
    assert!(!is_char_vowel('x'));
    assert!(!is_char_vowel('!'));
}
