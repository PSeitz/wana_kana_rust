use utils::is_char_in_range::*;

static KANJI_START: u32 = 0x4E00;
static KANJI_END: u32 = 0x9FAF;


///Tests a character. Returns true if the character is a CJK ideograph (kanji).
///@param  {String} char character string to test
///@return {Boolean}

pub fn is_char_kanji(char: char) -> bool {
    return is_char_in_range(char, KANJI_START, KANJI_END);
}
