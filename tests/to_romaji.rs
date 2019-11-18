#![feature(test)]
#![feature(non_ascii_idents)]

#[cfg(test)]
extern crate test;
extern crate wana_kana;

#[cfg(test)]
extern crate speculate;

#[cfg(test)]
use speculate::speculate;

use wana_kana::to_romaji::*;

speculate! {
    use wana_kana::Options;
    it "sane defaults" {
        assert_eq!(to_romaji(""), "");
    }

    it "Convert katakana to romaji" { assert_eq!(to_romaji("ワニカニ　ガ　スゴイ　ダ"), "wanikani ga sugoi da"); }

    it "Convert hiragana to romaji" {
        assert_eq!(to_romaji("わにかに　が　すごい　だ"), "wanikani ga sugoi da");
    }

    it "Convert mixed kana to romaji" { assert_eq!(to_romaji("ワニカニ　が　すごい　だ"), "wanikani ga sugoi da"); }

    // it "Will convert punctuation and full-width spaces" { assert_eq!(to_romaji(JA_PUNC.join('')), EN_PUNC.join('')); }

    it "Use the upcaseKatakana flag to preserve casing. Works for katakana." { assert_eq!(to_romaji_with_opt("ワニカニ", Options {upcase_katakana: true, ..Default::default() }), "WANIKANI"); }

    it "Use the upcaseKatakana flag to preserve casing. Works for mixed kana." {
      assert_eq!(to_romaji_with_opt("ワニカニ　が　すごい　だ", Options {upcase_katakana: true, ..Default::default() }), "WANIKANI ga sugoi da");
    }

    it "Converts long dash 'ー' in hiragana to hyphen" { assert_eq!(to_romaji("ばつげーむ"), "batsuge-mu"); }

    it "Doesn't confuse '一' (one kanji) for long dash 'ー'" { assert_eq!(to_romaji("一抹げーむ"), "一抹ge-mu"); }

    it "Converts long dash 'ー' (chōonpu) in katakana to long vowel" { assert_eq!(to_romaji("スーパー"), "suupaa"); }

    it "Doesn't convert オー to 'ou' which occurs with hiragana" { assert_eq!(to_romaji("缶コーヒー"), "缶koohii"); }

    it "Spaces must be manually entered" { assert_ne!(to_romaji("わにかにがすごいだ"), "wanikani ga sugoi da"); }

    describe "double n's and double consonants"{
      it "Double and single n" { assert_eq!(to_romaji("きんにくまん"), "kinnikuman"); }
      it "N extravaganza" { assert_eq!(to_romaji("んんにんにんにゃんやん"), "nnninninnyan'yan"); }
      it "Double consonants" {assert_eq!(to_romaji("かっぱ　たった　しゅっしゅ ちゃっちゃ　やっつ"),"kappa tatta shusshu chatcha yattsu"); }
      it "xxDouble consonants" {assert_eq!(to_romaji("かっぱ"),"kappa"); }
    }

    describe "Small kana" {
      it "Small tsu doesn't transliterate" { assert_eq!(to_romaji("っ"), ""); }
      it "Small kata ke doesn't transliterate" { assert_eq!(to_romaji("ヶ"), "ヶ"); }
      it "Small kata ka doesn't transliterate" { assert_eq!(to_romaji("ヵ"), "ヵ"); }
      it "Small ya" { assert_eq!(to_romaji("ゃ"), "ya"); }
      it "Small yu" { assert_eq!(to_romaji("ゅ"), "yu"); }
      it "Small yo" { assert_eq!(to_romaji("ょ"), "yo"); }
      it "Small a" { assert_eq!(to_romaji("ぁ"), "a"); }
      it "Small i" { assert_eq!(to_romaji("ぃ"), "i"); }
      it "Small u" { assert_eq!(to_romaji("ぅ"), "u"); }
      it "Small e" { assert_eq!(to_romaji("ぇ"), "e"); }
      it "Small o" { assert_eq!(to_romaji("ぉ"), "o"); }
    }

    describe "Apostrophes in ambiguous consonant vowel combos" {
      it "おんよみ" { assert_eq!(to_romaji("おんよみ"), "on'yomi"); }
      it "んよ んあ んゆ" { assert_eq!(to_romaji("んよ んあ んゆ"), "n'yo n'a n'yu"); }
      it "シンヨ" { assert_eq!(to_romaji("シンヨ"), "shin'yo"); }
    }

}
