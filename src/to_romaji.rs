//! Convert kana to romaji
//!
//! # Examples
//! ```
//! use wana_kana::to_romaji::*;
//! use wana_kana::Options;
//! assert_eq!(to_romaji("ひらがな　カタカナ"), "hiragana katakana");
//! assert_eq!(to_romaji_with_opt("ひらがな　カタカナ", Options {upcase_katakana: true, ..Default::default() } ), "hiragana KATAKANA");
//! ```

use crate::is_katakana::*;
use crate::options::Options;
use std;

use crate::utils::get_chunk::*;
use crate::utils::katakana_to_hiragana::*;

pub fn to_romaji(input: &str) -> String {
    to_romaji_with_opt(input, Options::default())
}
pub fn to_romaji_with_opt(input: &str, options: Options) -> String {
    let config = options;
    let kana = katakana_to_hiragana_with_opt(input, true);
    let len = kana.chars().count();
    // Final output array
    let mut roma = String::with_capacity(kana.len());
    // Position in the string that is being evaluated
    let mut cursor = 0;
    let max_chunk = 3;

    // let mut next_char_is_double_consonant = false;

    while cursor < len {
        let mut roma_char = None;
        let mut last_chunk = None;
        let mut chunk_size = std::cmp::min(max_chunk, len - cursor);
        let mut convert_romaji_to_uppercase = false;
        while chunk_size > 0 {
            let chunk = get_chunk(&kana, cursor, cursor + chunk_size);
            if is_katakana(&get_chunk(&input, cursor, cursor + chunk_size)) {
                convert_romaji_to_uppercase = config.upcase_katakana;
            }else{
                convert_romaji_to_uppercase = false;
            }
            // special case for small tsus
            // if chunk.chars().nth(0).map(|c| c == 'っ').unwrap_or(false) && chunk_size == 1 && cursor < (len - 1) {
            //     next_char_is_double_consonant = true;
            //     roma_char = Some(Cow::from(""));
            //     break;
            // }

            roma_char = TO_ROMAJI.get(&chunk as &str);
            if roma_char.is_some() {
                break;
            }
            // if let &mut Some(ref mut _roma_charo) = &mut roma_char {
                // if next_char_is_double_consonant {
                //     *roma_charo = Cow::from(roma_charo.chars().nth(0).unwrap().to_string() + &roma_charo);
                //     next_char_is_double_consonant = false;
                // }
            //     break;
            // }

            chunk_size -= 1;
            last_chunk = Some(chunk);
        }

        if let Some(roma_char) = roma_char {
            if convert_romaji_to_uppercase {
                roma.push_str(&roma_char.to_uppercase());
            }
            else{
                roma.push_str(&roma_char);
            }
        }
        else{
            roma.push_str(&last_chunk.unwrap());
        }

        // let roma_charo = roma_char.unwrap_or(&last_chunk.unwrap()); // Passthrough undefined values
        // if convert_romaji_to_uppercase {
        //     // roma_charo = Cow::from(roma_charo.to_uppercase());
        //     roma.push_str(&roma_charo.to_uppercase());
        // }else{
        //     roma.push_str(&roma_charo);
        // }

        cursor += std::cmp::max(chunk_size, 1);
    }
    roma
}


use fnv::FnvHashMap;

lazy_static! {

pub static ref TO_ROMAJI: FnvHashMap<&'static str, &'static str> = hashmap! {
  "あ" => "a",
  "い" => "i",
  "う" => "u",
  "え" => "e",
  "お" => "o",
  "か" => "ka",
  "き" => "ki",
  "きゃ" => "kya",
  "きゅ" => "kyu",
  "きょ" => "kyo",
  "きぃ" => "kyi",
  "きぇ" => "kye",
  "く" => "ku",
  "くゃ" => "kya",
  "くゅ" => "kyu",
  "くょ" => "kyo",
  "くぃ" => "kyi",
  "くぇ" => "kye",
  "け" => "ke",
  "こ" => "ko",
  "さ" => "sa",
  "し" => "shi",
  "しゃ" => "sha",
  "しゅ" => "shu",
  "しょ" => "sho",
  "しぃ" => "shyi",
  "しぇ" => "she",
  "す" => "su",
  "せ" => "se",
  "そ" => "so",
  "た" => "ta",
  "ち" => "chi",
  "ちゃ" => "cha",
  "ちゅ" => "chu",
  "ちょ" => "cho",
  "ちぃ" => "chyi",
  "ちぇ" => "che",
  "つ" => "tsu",
  "て" => "te",
  "と" => "to",
  "な" => "na",
  "に" => "ni",
  "にゃ" => "nya",
  "にゅ" => "nyu",
  "にょ" => "nyo",
  "にぃ" => "nyi",
  "にぇ" => "nye",
  "ぬ" => "nu",
  "ね" => "ne",
  "の" => "no",
  "は" => "ha",
  "ひ" => "hi",
  "ひゃ" => "hya",
  "ひゅ" => "hyu",
  "ひょ" => "hyo",
  "ひぃ" => "hyi",
  "ひぇ" => "hye",
  "ふ" => "fu",
  "ふゃ" => "fya",
  "ふゅ" => "fyu",
  "ふょ" => "fyo",
  "ふぃ" => "fyi",
  "ふぇ" => "fye",
  "へ" => "he",
  "ほ" => "ho",
  "ま" => "ma",
  "み" => "mi",
  "みゃ" => "mya",
  "みゅ" => "myu",
  "みょ" => "myo",
  "みぃ" => "myi",
  "みぇ" => "mye",
  "む" => "mu",
  "め" => "me",
  "も" => "mo",
  "ら" => "ra",
  "り" => "ri",
  "りゃ" => "rya",
  "りゅ" => "ryu",
  "りょ" => "ryo",
  "りぃ" => "ryi",
  "りぇ" => "rye",
  "る" => "ru",
  "れ" => "re",
  "ろ" => "ro",
  "や" => "ya",
  "ゆ" => "yu",
  "よ" => "yo",
  "わ" => "wa",
  "ゐ" => "wi",
  "ゑ" => "we",
  "を" => "wo",
  "ん" => "n",
  "んあ" => "n'a",
  "んい" => "n'i",
  "んう" => "n'u",
  "んえ" => "n'e",
  "んお" => "n'o",
  "んや" => "n'ya",
  "んゆ" => "n'yu",
  "んよ" => "n'yo",
  "が" => "ga",
  "ぎ" => "gi",
  "ぎゃ" => "gya",
  "ぎゅ" => "gyu",
  "ぎょ" => "gyo",
  "ぎぃ" => "gyi",
  "ぎぇ" => "gye",
  "ぐ" => "gu",
  "げ" => "ge",
  "ご" => "go",
  "ざ" => "za",
  "じ" => "ji",
  "じゃ" => "ja",
  "じゅ" => "ju",
  "じょ" => "jo",
  "じぃ" => "jyi",
  "じぇ" => "je",
  "ず" => "zu",
  "ぜ" => "ze",
  "ぞ" => "zo",
  "だ" => "da",
  "ぢ" => "ji",
  "ぢゃ" => "ja",
  "ぢゅ" => "ju",
  "ぢょ" => "jo",
  "ぢぃ" => "jyi",
  "ぢぇ" => "je",
  "づ" => "zu",
  "で" => "de",
  "ど" => "do",
  "ば" => "ba",
  "び" => "bi",
  "びゃ" => "bya",
  "びゅ" => "byu",
  "びょ" => "byo",
  "びぃ" => "byi",
  "びぇ" => "bye",
  "ぶ" => "bu",
  "べ" => "be",
  "ぼ" => "bo",
  "ぱ" => "pa",
  "ぴ" => "pi",
  "ぴゃ" => "pya",
  "ぴゅ" => "pyu",
  "ぴょ" => "pyo",
  "ぴぃ" => "pyi",
  "ぴぇ" => "pye",
  "ぷ" => "pu",
  "ぺ" => "pe",
  "ぽ" => "po",
  "ゔぁ" => "va",
  "ゔぃ" => "vyi",
  "ゔ" => "vu",
  "ゔゃ" => "vya",
  "ゔゅ" => "vyu",
  "ゔょ" => "vyo",
  "ゔぇ" => "ve",
  "ゔぉ" => "vo",
  "。" => ".",
  "、" => ",",
  "：" => ":",
  "・" => "/",
  "！" => "!",
  "？" => "?",
  "〜" => "~",
  "ー" => "-",
  "「" => "‘",
  "」" => "’",
  "『" => "“",
  "』" => "”",
  "［" => "[",
  "］" => "]",
  "（" => "(",
  "）" => ")",
  "｛" => "{",
  "｝" => "}",
  "　" => " ",
  "ゃ" => "ya",
  "ゅ" => "yu",
  "ょ" => "yo",
  "ぁ" => "a",
  "ぃ" => "i",
  "ぅ" => "u",
  "ぇ" => "e",
  "ぉ" => "o",
  "っあ" => "a",
  "っい" => "i",
  "っう" => "u",
  "っえ" => "e",
  "っお" => "o",
  "っか" => "kka",
  "っき" => "kki",
  "っきゃ" => "kkya",
  "っきゅ" => "kkyu",
  "っきょ" => "kkyo",
  "っきぃ" => "kkyi",
  "っきぇ" => "kkye",
  "っく" => "kku",
  "っくゃ" => "kkya",
  "っくゅ" => "kkyu",
  "っくょ" => "kkyo",
  "っくぃ" => "kkyi",
  "っくぇ" => "kkye",
  "っけ" => "kke",
  "っこ" => "kko",
  "っさ" => "ssa",
  "っし" => "sshi",
  "っしゃ" => "ssha",
  "っしゅ" => "sshu",
  "っしょ" => "ssho",
  "っしぃ" => "sshyi",
  "っしぇ" => "sshe",
  "っす" => "ssu",
  "っせ" => "sse",
  "っそ" => "sso",
  "った" => "tta",
  "っち" => "tchi",
  "っちゃ" => "tcha",
  "っちゅ" => "tchu",
  "っちょ" => "tcho",
  "っちぃ" => "tchyi",
  "っちぇ" => "tche",
  "っつ" => "ttsu",
  "って" => "tte",
  "っと" => "tto",
  "っな" => "na",
  "っに" => "ni",
  "っにゃ" => "nya",
  "っにゅ" => "nyu",
  "っにょ" => "nyo",
  "っにぃ" => "nyi",
  "っにぇ" => "nye",
  "っぬ" => "nu",
  "っね" => "ne",
  "っの" => "no",
  "っは" => "hha",
  "っひ" => "hhi",
  "っひゃ" => "hhya",
  "っひゅ" => "hhyu",
  "っひょ" => "hhyo",
  "っひぃ" => "hhyi",
  "っひぇ" => "hhye",
  "っふ" => "ffu",
  "っふゃ" => "ffya",
  "っふゅ" => "ffyu",
  "っふょ" => "ffyo",
  "っふぃ" => "ffyi",
  "っふぇ" => "ffye",
  "っへ" => "hhe",
  "っほ" => "hho",
  "っま" => "mma",
  "っみ" => "mmi",
  "っみゃ" => "mmya",
  "っみゅ" => "mmyu",
  "っみょ" => "mmyo",
  "っみぃ" => "mmyi",
  "っみぇ" => "mmye",
  "っむ" => "mmu",
  "っめ" => "mme",
  "っも" => "mmo",
  "っら" => "rra",
  "っり" => "rri",
  "っりゃ" => "rrya",
  "っりゅ" => "rryu",
  "っりょ" => "rryo",
  "っりぃ" => "rryi",
  "っりぇ" => "rrye",
  "っる" => "rru",
  "っれ" => "rre",
  "っろ" => "rro",
  "っや" => "ya",
  "っゆ" => "yu",
  "っよ" => "yo",
  "っわ" => "wwa",
  "っゐ" => "wwi",
  "っゑ" => "wwe",
  "っを" => "wwo",
  "っん" => "n",
  "っが" => "gga",
  "っぎ" => "ggi",
  "っぎゃ" => "ggya",
  "っぎゅ" => "ggyu",
  "っぎょ" => "ggyo",
  "っぎぃ" => "ggyi",
  "っぎぇ" => "ggye",
  "っぐ" => "ggu",
  "っげ" => "gge",
  "っご" => "ggo",
  "っざ" => "zza",
  "っじ" => "jji",
  "っじゃ" => "jja",
  "っじゅ" => "jju",
  "っじょ" => "jjo",
  "っじぃ" => "jjyi",
  "っじぇ" => "jje",
  "っず" => "zzu",
  "っぜ" => "zze",
  "っぞ" => "zzo",
  "っだ" => "dda",
  "っぢ" => "jji",
  "っぢゃ" => "jja",
  "っぢゅ" => "jju",
  "っぢょ" => "jjo",
  "っぢぃ" => "jjyi",
  "っぢぇ" => "jje",
  "っづ" => "zzu",
  "っで" => "dde",
  "っど" => "ddo",
  "っば" => "bba",
  "っび" => "bbi",
  "っびゃ" => "bbya",
  "っびゅ" => "bbyu",
  "っびょ" => "bbyo",
  "っびぃ" => "bbyi",
  "っびぇ" => "bbye",
  "っぶ" => "bbu",
  "っべ" => "bbe",
  "っぼ" => "bbo",
  "っぱ" => "ppa",
  "っぴ" => "ppi",
  "っぴゃ" => "ppya",
  "っぴゅ" => "ppyu",
  "っぴょ" => "ppyo",
  "っぴぃ" => "ppyi",
  "っぴぇ" => "ppye",
  "っぷ" => "ppu",
  "っぺ" => "ppe",
  "っぽ" => "ppo",
  "っゔぁ" => "vva",
  "っゔぃ" => "vvyi",
  "っゔ" => "vvu",
  "っゔゃ" => "vvya",
  "っゔゅ" => "vvyu",
  "っゔょ" => "vvyo",
  "っゔぇ" => "vve",
  "っゔぉ" => "vvo",
  "っ。" => ".",
  "っ、" => ",",
  "っ：" => ":",
  "っ・" => "/",
  "っ！" => "!",
  "っ？" => "?",
  "っ〜" => "~",
  "っー" => "-",
  "っ「" => "‘",
  "っ」" => "’",
  "っ『" => "“",
  "っ』" => "”",
  "っ［" => "[",
  "っ］" => "]",
  "っ（" => "(",
  "っ）" => ")",
  "っ｛" => "{",
  "っ｝" => "}",
  "っ　" => " ",
  "っゃ" => "ya",
  "っゅ" => "yu",
  "っょ" => "yo",
  "っぁ" => "a",
  "っぃ" => "i",
  "っぅ" => "u",
  "っぇ" => "e",
  "っぉ" => "o",
  "っ" => "",
};




}
