use utils::katakana_to_hiragana::*;
use utils::romaji_to_hiragana::romaji_to_hiragana;
use is_romaji::*;
use is_mixed::*;
use options::Options;
/**
 * Convert input to [Hiragana](https://en.wikipedia.org/wiki/Hiragana)
 * @param  {String} [input=''] text
 * @param  {DefaultOptions} [options=default_options]
 * @return {String} converted text
 * @example
 * to_hiragana('toukyou, オオサカ')
 * // => 'とうきょう、　おおさか'
 * to_hiragana('only カナ', { pass_romaji: true })
 * // => 'only かな'
 * to_hiragana('wi')
 * // => 'うぃ'
 * to_hiragana('wi', { use_obsolete_kana: true })
 * // => 'ゐ'
*/
pub fn to_hiragana(input: &str) -> String {
    to_hiragana_with_opt(input, Options::default())
}
pub fn to_hiragana_with_opt(input: &str, options: Options) -> String {
    let config = options;
    if config.pass_romaji {
        return katakana_to_hiragana(input);
    }
    if is_romaji(input) {
        return romaji_to_hiragana(input, config);
    }
    if is_mixed(input) {
        let romaji = katakana_to_hiragana(input);
        return romaji_to_hiragana(&romaji, config);
    }
    return katakana_to_hiragana(input);
}

#[test]
fn check_to_hiragana() {
    assert_eq!(
        to_hiragana("toukyou,オオサカ"),
        "とうきょう、おおさか"
    );
    assert_eq!(
        to_hiragana_with_opt(
            "only かな",
            Options {
                pass_romaji: true,
                ..Default::default()
            }
        ),
        "only かな"
    );
    assert_eq!(to_hiragana("wi"), "うぃ");
    assert_eq!(
        to_hiragana_with_opt(
            "wi",
            Options {
                use_obsolete_kana: true,
                ..Default::default()
            }
        ),
        "ゐ"
    );
}
