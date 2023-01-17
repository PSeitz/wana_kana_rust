/// Tests a character and an english consonant. Returns true if the char is a consonant.
///
/// * `include_Y` include y as a consonant in test

#[inline]
pub fn is_char_consonant(char: char, include_y: bool) -> bool {
    match char {
        'b' | 'c' | 'd' | 'f' | 'g' | 'h' | 'j' | 'k' | 'l' | 'm' | 'n' | 'p' | 'q' | 'r' | 's'
        | 't' | 'v' | 'w' | 'x' | 'z' => true,
        'y' if include_y => true,
        _ => false,
    }
}

#[test]
fn is_char_consonant_test() {
    assert_eq!(is_char_consonant('y', false), false);
    assert_eq!(is_char_consonant('y', true), true);
    assert_eq!(is_char_consonant('a', true), false);
    assert_eq!(is_char_consonant('!', true), false);
}
