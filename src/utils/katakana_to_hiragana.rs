use constants::{HIRAGANA_START, KATAKANA_START, LONG_VOWELS, TO_ROMAJI};
use utils::is_char_long_dash::*;
use utils::is_char_slash_dot::*;
use utils::is_char_katakana::*;
use std;
/**
 * Convert [Katakana](https://en.wikipedia.org/wiki/Katakana) to [Hiragana](https://en.wikipedia.org/wiki/Hiragana)
 * Passes through any non-katakana chars
 * @param  {String} [input=''] text input
 * @return {String} converted text
 * @example
 * katakana_to_hiragana('カタカナ')
 * // => "かたかな"
 * katakana_to_hiragana('カタカナ is a type of kana')
 * // => "かたかな is a type of kana"
 * @ignore
 */
fn katakana_to_hiragana(input: &str) -> String {
    let mut hira = vec![];
    let mut previous_kana: Option<char> = None; //"".to_string();
                                                // let iterable = ;
                                                // for (let index = 0; index < iterable.length; index += 1) {
    for (index, char) in input.chars().enumerate() {
        // let char = iterable[index];
        let [slash_dot, long_dash] = [is_char_slash_dot(char), is_char_long_dash(char)];
        // Short circuit to avoid incorrect codeshift for 'ー' and '・'
        if slash_dot || (long_dash && index < 1) {
            hira.push(char);
        // Transform long vowels: 'オー' to 'おう'
        } else if let (Some(previous_kana), true, true) = (previous_kana, index > 0, long_dash) {
            // else if previous_kana.is_some() && long_dash && index > 0 {
            // Transform previous_kana back to romaji, and slice off the vowel
            let romaji = TO_ROMAJI[&previous_kana.to_string() as &str];
            hira.push(LONG_VOWELS[&romaji[0..romaji.len() - 1].chars().next().unwrap()]);
        } else if !long_dash && is_char_katakana(char) {
            // Shift charcode.
            let code = char as i32 + (HIRAGANA_START as i32 - KATAKANA_START as i32) as i32;
            let hira_char = std::char::from_u32(code as u32).unwrap();
            // hira.push(hira_char);
            hira.push(hira_char);
            previous_kana = Some(hira_char);
        } else {
            // Pass non katakana chars through
            hira.push(char);
            previous_kana = None;
        }
    }
    // return hira.join('');
    hira.into_iter().collect()
}

#[test]
fn test_katakana_to_hiragana() {
    assert_eq!(katakana_to_hiragana("カタカナ"), "かたかな");
    assert_eq!(
        katakana_to_hiragana("カタカナ is a type of kana"),
        "かたかな is a type of kana"
    );
}
