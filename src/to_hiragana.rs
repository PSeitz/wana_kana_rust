use crate::is_mixed::*;
use crate::is_romaji::*;
use crate::options::Options;
use crate::utils::is_char_english_punctuation::is_char_english_punctuation;
use crate::utils::katakana_to_hiragana::*;
use crate::utils::romaji_to_hiragana::romaji_to_hiragana;

#[inline]
/// Convert input to [Hiragana](https://en.wikipedia.org/wiki/Hiragana)
pub fn to_hiragana(input: &str) -> String {
    to_hiragana_with_opt(input, Options::default())
}

/// Convert input to [Hiragana](https://en.wikipedia.org/wiki/Hiragana)
pub fn to_hiragana_with_opt(input: &str, options: Options) -> String {
    let config = options;
    if config.pass_romaji {
        katakana_to_hiragana(input)
    } else if is_mixed(input) {
        let romaji = katakana_to_hiragana(input);
        romaji_to_hiragana(&romaji, config)
    } else if is_romaji(input)
        || input
            .chars()
            .next()
            .map(is_char_english_punctuation)
            .unwrap_or(false)
    {
        // TODO: is it correct to check only the first char (see
        // src\utils\isCharEnglishPunctuation.js)
        romaji_to_hiragana(input, config)
    } else {
        katakana_to_hiragana(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn with_obsolete_kana() -> Options {
        Options {
            use_obsolete_kana: true,
            ..Default::default()
        }
    }

    #[test]
    fn sane_defaults() {
        assert_eq!(to_hiragana(""), "");
    }

    #[test]
    fn quick_brown_fox_romaji_to_hiragana() {
        // https://en.wikipedia.org/wiki/Iroha
        // Even the colorful fragrant flowers'
        assert_eq!(
            to_hiragana_with_opt("IROHANIHOHETO", with_obsolete_kana()),
            "いろはにほへと"
        );
        // die sooner or later.'
        assert_eq!(
            to_hiragana_with_opt("CHIRINURUWO", with_obsolete_kana()),
            "ちりぬるを"
        );
        // Us who live in this world'
        assert_eq!(
            to_hiragana_with_opt("WAKAYOTARESO", with_obsolete_kana()),
            "わかよたれそ"
        );
        // cannot live forever, either.'
        assert_eq!(
            to_hiragana_with_opt("TSUNENARAMU", with_obsolete_kana()),
            "つねならむ"
        );
        // This transient mountain with shifts and changes,'
        assert_eq!(
            to_hiragana_with_opt("UWINOOKUYAMA", with_obsolete_kana()),
            "うゐのおくやま"
        );
        // today we are going to overcome, and reach the world of enlightenment.'
        assert_eq!(
            to_hiragana_with_opt("KEFUKOETE", with_obsolete_kana()),
            "けふこえて"
        );
        // We are not going to have meaningless dreams'
        assert_eq!(
            to_hiragana_with_opt("ASAKIYUMEMISHI", with_obsolete_kana()),
            "あさきゆめみし"
        );
        // nor become intoxicated with the fake world anymore.'
        assert_eq!(
            to_hiragana_with_opt("WEHIMOSESU", with_obsolete_kana()),
            "ゑひもせす"
        );
        // *not in iroha*
        assert_eq!(to_hiragana("NLTU"), "んっ");
    }

    mod use_obsolete_kana {
        use super::*;
        #[test]
        fn use_obsolete_kana_is_false_by_default() {
            assert_eq!(to_hiragana("wi"), "うぃ");
        }
        #[test]
        fn wi_ゐ_when_use_obsolete_kana_is_true() {
            assert_eq!(to_hiragana_with_opt("wi", with_obsolete_kana()), "ゐ");
        }
        #[test]
        fn we_ゑ_when_use_obsolete_kana_is_true() {
            assert_eq!(to_hiragana_with_opt("we", with_obsolete_kana()), "ゑ");
        }
        #[test]
        fn wi_うぃ_when_use_obsolete_kana_is_false() {
            assert_eq!(
                to_hiragana_with_opt(
                    "wi",
                    Options {
                        use_obsolete_kana: false,
                        ..Default::default()
                    }
                ),
                "うぃ"
            );
        }
    }

    mod pass_romaji {
        use super::*;
        #[test]
        fn false_by_default() {
            assert_eq!(to_hiragana("only カナ"), "おんly かな");
            assert_eq!(
                to_hiragana_with_opt(
                    "only カナ",
                    Options {
                        pass_romaji: true,
                        ..Default::default()
                    }
                ),
                "only かな"
            );
        }
    }

    mod katakana_choōnpu {
        use super::*;
        #[test]
        fn converts_to_hiragana_long_vowels() {
            assert_eq!(to_hiragana("スーパー"), "すうぱあ");
            assert_eq!(to_hiragana("バンゴー"), "ばんごう");
        }
    }

    #[test]
    fn mixed_input() {
        assert_eq!(
            to_hiragana("#22 ２２漢字、toukyou, オオサカ"),
            "#22 ２２漢字、とうきょう、 おおさか"
        );
    }
}
