use utils::is_char_kana::*;
use utils::is_char_punctuation::*;
use is_japanese::*;
use is_kana::*;
use is_kanji::*;


///Strips trailing [Okurigana](https://en.wikipedia.org/wiki/Okurigana) if `input` is a mix of [Kanji](https://en.wikipedia.org/wiki/Kanji) and [Kana](https://en.wikipedia.org/wiki/Kana)
///
///@param  {String} input text
///
///@param  {Object} [options={ all: false }] config object specifying if *all* kana should be removed, not just trailing okurigana
///
/// # Examples
///
///strip_okurigana('踏み込む')
///
/// => '踏み込'
///
///strip_okurigana('粘り。')
///
/// => '粘。'
///
///strip_okurigana('お祝い')
///
/// => 'お祝'
///
///strip_okurigana('踏み込む', { all: true })
///
/// => '踏込'
///
///strip_okurigana('お祝い', { all: true })
///
/// => '祝'
///

pub fn strip_okurigana(input: &str) -> String {
    strip_okurigana_all(input, false)
}
pub fn strip_okurigana_all(input: &str, all: bool) -> String {
    if input.is_empty() || !is_japanese(input) || is_kana(input) {
        return input.to_string();
    }

    if all {
        return input
            .chars()
            .filter(|char| !is_char_kana(*char))
            .into_iter()
            .collect();
    }

    // strip trailing only
    let mut reverse_chars = input.chars().rev().collect::<Vec<char>>();

    let mut i = 0;
    while i != reverse_chars.len() {
        let char = reverse_chars[i];
        if is_char_punctuation(char) {
            i += 1;
            continue;
        }

        if !is_kanji(&char.to_string()) {
            reverse_chars.remove(i);
        } else {
            break; // stop when we hit a kanji char
        }
    }

    return reverse_chars.into_iter().rev().collect();
}

#[test]
fn check_to_strip_okurigana() {
    assert_eq!(strip_okurigana_all("踏み込む", false), "踏み込");
    assert_eq!(strip_okurigana_all("粘り。", true), "粘。");
    assert_eq!(strip_okurigana_all("お祝い", false), "お祝");
    assert_eq!(strip_okurigana_all("踏み込む", true), "踏込");
    assert_eq!(strip_okurigana_all("お祝い", true), "祝");
}
