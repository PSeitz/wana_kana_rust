#![feature(test)]
#![feature(non_ascii_idents)]

#[cfg(test)]
extern crate test;
extern crate wana_kana;

#[cfg(test)]
extern crate speculate;

#[cfg(test)]
use speculate::speculate;

use wana_kana::Options;
use wana_kana::to_kana::*;

fn with_obsolete_kana() -> Options {
    Options{ use_obsolete_kana: true, .. Default::default() }
}

fn with_ime_mode() -> Options {
    Options{ use_obsolete_kana: true, .. Default::default() }
}


speculate!{

    it "sane defaults" {
        assert_eq!(to_kana(""), "");
    }
    it "uppercase ist katakana defaults" {
        assert_eq!(to_kana("WANAKANA"), "ワナカナ");
    }

    it "Lowercase characters are transliterated to hiragana." { assert_eq!(to_kana("onaji"), "おなじ"); }

    it "Lowercase with double consonants and double vowels are transliterated to hiragana." { assert_eq!(to_kana("buttsuuji"), "ぶっつうじ"); }

    it "Uppercase characters are transliterated to katakana." { assert_eq!(to_kana("ONAJI"), "オナジ"); }

    it "Uppercase with double consonants and double vowels are transliterated to katakana." { assert_eq!(to_kana("BUTTSUUJI"), "ブッツウジ"); }

    it "WaniKani -> わにかに - Mixed case returns hiragana (katakana only if all letters of mora are uppercased)." { assert_eq!(to_kana("WaniKani"), "わにかに"); }

    it "Non-romaji will be passed through." { assert_eq!(to_kana("ワニカニ AiUeO 鰐蟹 12345 @#$%"), "ワニカニ アいウえオ 鰐蟹 12345 @#$%"); }

    it "It handles mixed syllabaries" { assert_eq!(to_kana("座禅‘zazen’スタイル"), "座禅「ざぜん」スタイル"); }

    it "Will convert short to long dashes" { assert_eq!(to_kana("batsuge-mu"), "ばつげーむ"); }

    // it "Will convert punctuation but pass through spaces" { assert_eq!(to_kana(EN_PUNC.join(' '))).toBe(JA_PUNC.join(' ')));

    describe "without IME Mode" {
      it "solo n's are transliterated regardless of following chars" {
        assert_eq!(to_kana("n"), "ん");
        assert_eq!(to_kana("shin"), "しん");
      }
      it "double n's are transliterated to double ん" { assert_eq!(to_kana("nn"), "んん"); }
    }

    describe "with IME Mode" {
      it "solo n's are not transliterated unless chars follow" {
        assert_eq!(to_kana_with_opt("n", with_ime_mode()), "n" );
        assert_eq!(to_kana_with_opt("shin", with_ime_mode()), "しn" );
        assert_eq!(to_kana_with_opt("shinyou", with_ime_mode()), "しにょう" );
        assert_eq!(to_kana_with_opt("shin'you", with_ime_mode()), "しんよう" );
        assert_eq!(to_kana_with_opt("shin you", with_ime_mode()), "しんよう" );
      }
      it "double n's are transliterated to single　ん" { assert_eq!(to_kana_with_opt("nn", with_ime_mode()), "ん" ); }
    }

    describe "useObsoleteKana" {
      it "useObsoleteKana is false by default" {
        assert_eq!(to_kana("wi"), "うぃ");
        assert_eq!(to_kana("WI"), "ウィ");
      }
      it "wi = ゐ (when useObsoleteKana is true)" { assert_eq!(to_kana_with_opt("wi", with_obsolete_kana()), "ゐ"); }
      it "we = ゑ (when useObsoleteKana is true)" { assert_eq!(to_kana_with_opt("we", with_obsolete_kana()), "ゑ"); }
      it "WI = ヰ (when useObsoleteKana is true)" { assert_eq!(to_kana_with_opt("WI", with_obsolete_kana()), "ヰ"); }
      it "WE = ヱ (when useObsoleteKana is true)" { assert_eq!(to_kana_with_opt("WE", with_obsolete_kana()), "ヱ"); }
    }

}











// describe "optionso" {
//     describe "use_obsolete_kana" {
//         // describe "to_kana" {
//         //     it "use_obsolete_kana is false by default" {
//         //         assert_eq!(to_kana("wi"), "うぃ");
//         //     }
//         //     it "wi = ゐ (when use_obsolete_kana is true)" {
//         //         assert_eq!(to_kana_with_opt("wi", Options{ use_obsolete_kana: true, .. Default::default() }), "ゐ");
//         //     }
//         //     it "we = ゑ (when use_obsolete_kana is true)" {
//         //         assert_eq!(to_kana_with_opt("we", Options{ use_obsolete_kana: true, .. Default::default() }), "ゑ");
//         //     }
//         //     it "WI = ヰ (when use_obsolete_kana is true)" {
//         //         assert_eq!(to_kana_with_opt("WI", Options{ use_obsolete_kana: true, .. Default::default() }), "ヰ");
//         //     }
//         //     it "WE = ヱ (when use_obsolete_kana is true)" {
//         //         assert_eq!(to_kana_with_opt("WE", Options{ use_obsolete_kana: true, .. Default::default() }), "ヱ");
//         //     }
//         // }

//         describe "to_hiragana" {
//             it "use_obsolete_kana is false by default" {
//                 assert_eq!(to_hiragana("wi"), "うぃ");
//             }
//             it "wi = ゐ (when use_obsolete_kana is true)" {
//                 assert_eq!(to_hiragana_with_opt("wi", Options{ use_obsolete_kana: true, .. Default::default() }), "ゐ");
//             }
//             it "we = ゑ (when use_obsolete_kana is true)" {
//                 assert_eq!(to_hiragana_with_opt("we", Options{ use_obsolete_kana: true, .. Default::default() }), "ゑ");
//             }
//             it "wi = うぃ when use_obsolete_kana is false" {
//                 assert_eq!(to_hiragana_with_opt("wi", Options{ use_obsolete_kana: false, .. Default::default() }), "うぃ");
//             }
//         }

//         describe "to_kata_kana" {
//             it "wi = ウィ when use_obsolete_kana is false" {
//                 assert_eq!(to_katakana_with_opt("WI", Options{ use_obsolete_kana: false, .. Default::default() }), "ウィ");
//             }
//             it "WI = ヰ (when use_obsolete_kana is true)" {
//                 assert_eq!(to_katakana_with_opt("wi", Options{ use_obsolete_kana: true, .. Default::default() }), "ヰ");
//             }
//             it "WE = ヱ (when use_obsolete_kana is true)" {
//                 assert_eq!(to_katakana_with_opt("we", Options{ use_obsolete_kana: true, .. Default::default() }), "ヱ");
//             }
//         }
//     }
// }
