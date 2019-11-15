//! Convert [Romaji](https://en.wikipedia.org/wiki/Romaji) to [Kana](https://en.wikipedia.org/wiki/Kana), lowercase text will result in [Hiragana](https://en.wikipedia.org/wiki/Hiragana) and uppercase text will result in [Katakana](https://en.wikipedia.org/wiki/Katakana).
//!
//! # Examples
//! ```
//! use wana_kana::to_kana::*;
//! use wana_kana::Options;
//! assert_eq!(to_kana_with_opt("o", Options::default()), "お");
//! assert_eq!(to_kana_with_opt("ona", Options::default()), "おな");
//! //assert_eq!(to_kana_with_opt("onaji", Options::default()), "おなじ");
//! //assert_eq!(to_kana_with_opt("onaji BUTTSUUJI", Options::default()), "おなじ ブッツウジ");
//! //assert_eq!(to_kana_with_opt("ONAJI buttsuuji", Options::default()), "オナジ ぶっつうじ");
//! assert_eq!(to_kana_with_opt("座禅‘zazen’スタイル", Options::default()), "座禅「ざぜん」スタイル");
//! assert_eq!(to_kana_with_opt("batsuge-mu", Options {use_obsolete_kana: true, ..Default::default() } ), "ばつげーむ");
//! assert_eq!(to_kana_with_opt("!?./,~-‘’“”[](){}", Options::default()), "！？。・、〜ー「」『』［］（）｛｝");
//! assert_eq!(to_kana_with_opt("we", Options {use_obsolete_kana: true, ..Default::default() } ), "ゑ");
//! ```


use crate::options::Options;
use std;

use crate::utils::get_chunk::get_chunk;
use crate::utils::hiragana_to_katakana::*;


pub fn to_kana(input: &str) -> String {
    to_kana_with_opt(input, Options::default())
}

pub fn to_kana_with_opt(orig: &str, options: Options) -> String {
    // let config = options;
    let input = &orig.to_lowercase();
    let len = input.chars().count();
    // Final output array
    let mut ouput = String::with_capacity(input.len());
    // Position in the string that is being evaluated
    let mut cursor = 0;
    let max_chunk = 4;

    while cursor < len {
        let mut chunk_size = std::cmp::min(max_chunk, len - cursor);
        loop {
            let chunk = get_chunk(&input, cursor, cursor + chunk_size);

            let mapping = if options.use_obsolete_kana {
                TO_KANA_OBSOLETE.get(&chunk as &str)
            }else if options.imemode{
                TO_KANA_IMEMODE.get(&chunk as &str)
            }else{
                TO_KANA.get(&chunk as &str)
            };

            if let Some(mapping) = mapping {
                if get_chunk(&orig, cursor, cursor + chunk_size).chars().all(char::is_uppercase){
                  ouput.push_str(&hiragana_to_katakana(mapping));
                }else{
                  ouput.push_str(&mapping);
                }
                break;
            }

            chunk_size -= 1;
            if chunk_size == 0 {
                ouput.push_str(&chunk);
                break;
            }
        };

        cursor += std::cmp::max(chunk_size, 1);
    }
    ouput
}

// pub fn to_kana_with_opt_2(input: &str, options: Options) -> String {
//     let config = options;
//     // Final output array containing arrays [start index of the translitterated substring, end index, kana]
//     let mut kana = String::with_capacity(input.len());
//     // Position in the string that is being evaluated
//     let mut cursor = 0;
//     let len = input.chars().count();
//     let max_chunk = 3;
//     // let mut chunk_size = 3;
//     // Steps through the string pulling out chunks of characters. Each chunk will be evaluated
//     // against the romaji to kana table. If there is no match, the last character in the chunk
//     // is dropped and the chunk is reevaluated. If nothing matches, the character is assumed
//     // to be invalid or punctuation or other and gets passed through.
//     while cursor < len {
//         let mut chunk = Cow::from("");
//         let mut chunk_lc = Cow::from("");
//         let mut kana_char = Cow::from("");
//         let mut chunk_size = std::cmp::min(max_chunk, len - cursor);
//         while chunk_size > 0 {
//             chunk = Cow::from(get_chunk(input, cursor, cursor + chunk_size));
//             chunk_lc = lower_cow(&chunk);
//             // Handle super-rare edge cases with 4 char chunks (like ltsu, chya, shya)
//             if FOUR_CHAR_EDGECASES.contains(&(&chunk_lc as &str)) && len - cursor >= 4 {
//                 chunk_size += 1;
//                 chunk = Cow::from(get_chunk(input, cursor, cursor + chunk_size));
//                 chunk_lc = lower_cow(&chunk);
//             } else if let (Some(lc), Some(c)) = (chunk_lc.chars().nth(0), chunk.chars().nth(0)) {
//                 // Handle edge case of n followed by consonant
//                 if lc == 'n' {
//                     if chunk_size == 2 {
//                         // Handle edge case of n followed by a space (only if not in IME mode)
//                         if !config.imemode && chunk_lc.chars().nth(1).map(|c| c == ' ').unwrap_or(false) {
//                             kana_char = Cow::from("ん ");
//                             break;
//                         }
//                         // Convert IME input of n' to "ん"
//                         if config.imemode && chunk_lc == "n'" {
//                             kana_char = Cow::from("ん");
//                             break;
//                         }
//                     }
//                     // Handle edge case of n followed by n and vowel
//                     if chunk_lc.chars().nth(1).map(|c| is_char_consonant(c, false)).unwrap_or(false)
//                         && chunk_lc.chars().nth(2).map(is_char_vowel).unwrap_or(false)
//                     {
//                         chunk_size = 1;
//                         chunk = Cow::from(get_chunk(input, cursor, cursor + chunk_size));
//                         chunk_lc = lower_cow(&chunk);
//                     }
//                 }

//                 // Handle case of double consonants
//                 if lc != 'n' && is_char_consonant(lc, true) && Some(c) == chunk.chars().nth(1) {
//                     chunk_size = 1;
//                     // Return katakana ッ if chunk is uppercase, otherwise return hiragana っ
//                     if is_char_in_range(c, UPPERCASE_START, UPPERCASE_END) {
//                         chunk_lc = Cow::from("ッ");
//                         chunk = Cow::from("ッ");
//                     } else {
//                         chunk_lc = Cow::from("っ");
//                         chunk = Cow::from("っ");
//                     }
//                 }
//             }

//             if let Some(char) = FROM_ROMAJI.get(&chunk_lc as &str) {
//                 kana_char = Cow::from(*char);
//                 break;
//             } else {
//                 kana_char = Cow::from("");
//             }
//             // Step down the chunk size.
//             // If chunk_size was 4, step down twice.
//             if chunk_size == 4 {
//                 chunk_size -= 2;
//             } else {
//                 chunk_size -= 1;
//             }
//         }

//         // Passthrough undefined values
//         if kana_char == "" {
//             kana_char = chunk.clone();
//         }

//         // Handle special cases.
//         if config.use_obsolete_kana {
//             if chunk_lc == "wi" {
//                 kana_char = Cow::from("ゐ")
//             };
//             if chunk_lc == "we" {
//                 kana_char = Cow::from("ゑ")
//             };
//         }

//         if config.imemode && chunk_lc.chars().nth(0).map(|c| c == 'n').unwrap_or(false) {
//             if input
//                 .chars()
//                 .nth(cursor + 1)
//                 .map(|c| c.to_string().to_lowercase() == "y")
//                 .unwrap_or(false)
//                 && input.chars().nth(cursor + 2).map(|c| !is_char_vowel(c)).unwrap_or(true)
//                 || cursor == len - 1
//                 || input.chars().nth(cursor + 1).map(|c| is_kana(&c.to_string())).unwrap_or(false)
//             {
//                 // Don't transliterate this yet.
//                 kana_char = Cow::from(chunk.chars().nth(0).unwrap().to_string());
//             }
//         }

//         // Use katakana if first letter in chunk is uppercase
//         if chunk.chars().nth(0).map(|c| is_char_upper_case(c)).unwrap_or(false) {
//             kana_char = Cow::from(hiragana_to_katakana(&kana_char));
//         }

//         cursor += std::cmp::max(chunk_size, 1);

//         kana.push_str(&kana_char);
//     }
//     return kana;
// }

use fnv::FnvHashMap;
lazy_static! {

pub static ref TO_KANA_OBSOLETE: FnvHashMap<&'static str, &'static str> = {
    let mut map: FnvHashMap<_, _> = TO_KANA.clone();
    map.insert("wi", "ゐ");
    map.insert("we", "ゑ");
    map
};

pub static ref TO_KANA_IMEMODE: FnvHashMap<&'static str, &'static str> = {
    let mut map: FnvHashMap<_, _> = TO_KANA.clone();
    map.insert("nn", "ん");
    map.insert("n ", "ん");
    map
};

pub static ref TO_KANA: FnvHashMap<&'static str, &'static str> = hashmap! {
  "a" => "あ",
  "i" => "い",
  "u" => "う",
  "e" => "え",
  "o" => "お",
  "ka" => "か",
  "ki" => "き",
  "ku" => "く",
  "ke" => "け",
  "ko" => "こ",
  "kya" => "きゃ",
  "kyi" => "きぃ",
  "kyu" => "きゅ",
  "kye" => "きぇ",
  "kyo" => "きょ",
  "kwa" => "くぁ",
  "kka" => "っか",
  "kki" => "っき",
  "kku" => "っく",
  "kke" => "っけ",
  "kko" => "っこ",
  "kkya" => "っきゃ",
  "kkyi" => "っきぃ",
  "kkyu" => "っきゅ",
  "kkye" => "っきぇ",
  "kkyo" => "っきょ",
  "kkwa" => "っくぁ",
  "sa" => "さ",
  "si" => "し",
  "su" => "す",
  "se" => "せ",
  "so" => "そ",
  "sya" => "しゃ",
  "syi" => "しぃ",
  "syu" => "しゅ",
  "sye" => "しぇ",
  "syo" => "しょ",
  "swa" => "すぁ",
  "swi" => "すぃ",
  "swu" => "すぅ",
  "swe" => "すぇ",
  "swo" => "すぉ",
  "sha" => "しゃ",
  "shi" => "し",
  "shu" => "しゅ",
  "she" => "しぇ",
  "sho" => "しょ",
  "shya" => "しゃ",
  "shyi" => "しぃ",
  "shyu" => "しゅ",
  "shye" => "しぇ",
  "shyo" => "しょ",
  "ssa" => "っさ",
  "ssi" => "っし",
  "ssu" => "っす",
  "sse" => "っせ",
  "sso" => "っそ",
  "ssya" => "っしゃ",
  "ssyi" => "っしぃ",
  "ssyu" => "っしゅ",
  "ssye" => "っしぇ",
  "ssyo" => "っしょ",
  "sswa" => "っすぁ",
  "sswi" => "っすぃ",
  "sswu" => "っすぅ",
  "sswe" => "っすぇ",
  "sswo" => "っすぉ",
  "ssha" => "っしゃ",
  "sshi" => "っし",
  "sshu" => "っしゅ",
  "sshe" => "っしぇ",
  "ssho" => "っしょ",
  "sshya" => "っしゃ",
  "sshyi" => "っしぃ",
  "sshyu" => "っしゅ",
  "sshye" => "っしぇ",
  "sshyo" => "っしょ",
  "ta" => "た",
  "ti" => "ち",
  "tu" => "つ",
  "te" => "て",
  "to" => "と",
  "tya" => "ちゃ",
  "tyi" => "ちぃ",
  "tyu" => "ちゅ",
  "tye" => "ちぇ",
  "tyo" => "ちょ",
  "tsa" => "つぁ",
  "tsi" => "つぃ",
  "tsu" => "つ",
  "tse" => "つぇ",
  "tso" => "つぉ",
  "tha" => "てゃ",
  "thi" => "てぃ",
  "thu" => "てゅ",
  "the" => "てぇ",
  "tho" => "てょ",
  "twa" => "とぁ",
  "twi" => "とぃ",
  "twu" => "とぅ",
  "twe" => "とぇ",
  "two" => "とぉ",
  "tta" => "った",
  "tti" => "っち",
  "ttu" => "っつ",
  "tte" => "って",
  "tto" => "っと",
  "ttya" => "っちゃ",
  "ttyi" => "っちぃ",
  "ttyu" => "っちゅ",
  "ttye" => "っちぇ",
  "ttyo" => "っちょ",
  "ttsa" => "っつぁ",
  "ttsi" => "っつぃ",
  "ttsu" => "っつ",
  "ttse" => "っつぇ",
  "ttso" => "っつぉ",
  "ttha" => "ってゃ",
  "tthi" => "ってぃ",
  "tthu" => "ってゅ",
  "tthe" => "ってぇ",
  "ttho" => "ってょ",
  "ttwa" => "っとぁ",
  "ttwi" => "っとぃ",
  "ttwu" => "っとぅ",
  "ttwe" => "っとぇ",
  "ttwo" => "っとぉ",
  "na" => "な",
  "ni" => "に",
  "nu" => "ぬ",
  "ne" => "ね",
  "no" => "の",
  "nya" => "にゃ",
  "nyi" => "にぃ",
  "nyu" => "にゅ",
  "nye" => "にぇ",
  "nyo" => "にょ",
  "n" => "ん",
  "n'" => "ん",
  "ha" => "は",
  "hi" => "ひ",
  "hu" => "ふ",
  "he" => "へ",
  "ho" => "ほ",
  "hya" => "ひゃ",
  "hyi" => "ひぃ",
  "hyu" => "ひゅ",
  "hye" => "ひぇ",
  "hyo" => "ひょ",
  "hha" => "っは",
  "hhi" => "っひ",
  "hhu" => "っふ",
  "hhe" => "っへ",
  "hho" => "っほ",
  "hhya" => "っひゃ",
  "hhyi" => "っひぃ",
  "hhyu" => "っひゅ",
  "hhye" => "っひぇ",
  "hhyo" => "っひょ",
  "ma" => "ま",
  "mi" => "み",
  "mu" => "む",
  "me" => "め",
  "mo" => "も",
  "mya" => "みゃ",
  "myi" => "みぃ",
  "myu" => "みゅ",
  "mye" => "みぇ",
  "myo" => "みょ",
  "mma" => "っま",
  "mmi" => "っみ",
  "mmu" => "っむ",
  "mme" => "っめ",
  "mmo" => "っも",
  "mmya" => "っみゃ",
  "mmyi" => "っみぃ",
  "mmyu" => "っみゅ",
  "mmye" => "っみぇ",
  "mmyo" => "っみょ",
  "ya" => "や",
  "yu" => "ゆ",
  "yo" => "よ",
  "yi" => "い",
  "ye" => "いぇ",
  "yya" => "っや",
  "yyu" => "っゆ",
  "yyo" => "っよ",
  "yyi" => "っい",
  "yye" => "っいぇ",
  "ra" => "ら",
  "ri" => "り",
  "ru" => "る",
  "re" => "れ",
  "ro" => "ろ",
  "rya" => "りゃ",
  "ryi" => "りぃ",
  "ryu" => "りゅ",
  "rye" => "りぇ",
  "ryo" => "りょ",
  "rra" => "っら",
  "rri" => "っり",
  "rru" => "っる",
  "rre" => "っれ",
  "rro" => "っろ",
  "rrya" => "っりゃ",
  "rryi" => "っりぃ",
  "rryu" => "っりゅ",
  "rrye" => "っりぇ",
  "rryo" => "っりょ",
  "wa" => "わ",
  "wi" => "うぃ",
  "we" => "うぇ",
  "wo" => "を",
  "wha" => "うぁ",
  "whi" => "うぃ",
  "whu" => "う",
  "whe" => "うぇ",
  "who" => "うぉ",
  "wu" => "う",
  "wwa" => "っわ",
  "wwi" => "っうぃ",
  "wwe" => "っうぇ",
  "wwo" => "っを",
  "wwha" => "っうぁ",
  "wwhi" => "っうぃ",
  "wwhu" => "っう",
  "wwhe" => "っうぇ",
  "wwho" => "っうぉ",
  "wwu" => "っう",
  "ga" => "が",
  "gi" => "ぎ",
  "gu" => "ぐ",
  "ge" => "げ",
  "go" => "ご",
  "gya" => "ぎゃ",
  "gyi" => "ぎぃ",
  "gyu" => "ぎゅ",
  "gye" => "ぎぇ",
  "gyo" => "ぎょ",
  "gwa" => "ぐぁ",
  "gwi" => "ぐぃ",
  "gwu" => "ぐぅ",
  "gwe" => "ぐぇ",
  "gwo" => "ぐぉ",
  "gga" => "っが",
  "ggi" => "っぎ",
  "ggu" => "っぐ",
  "gge" => "っげ",
  "ggo" => "っご",
  "ggya" => "っぎゃ",
  "ggyi" => "っぎぃ",
  "ggyu" => "っぎゅ",
  "ggye" => "っぎぇ",
  "ggyo" => "っぎょ",
  "ggwa" => "っぐぁ",
  "ggwi" => "っぐぃ",
  "ggwu" => "っぐぅ",
  "ggwe" => "っぐぇ",
  "ggwo" => "っぐぉ",
  "za" => "ざ",
  "zi" => "じ",
  "zu" => "ず",
  "ze" => "ぜ",
  "zo" => "ぞ",
  "zya" => "じゃ",
  "zyi" => "じぃ",
  "zyu" => "じゅ",
  "zye" => "じぇ",
  "zyo" => "じょ",
  "zza" => "っざ",
  "zzi" => "っじ",
  "zzu" => "っず",
  "zze" => "っぜ",
  "zzo" => "っぞ",
  "zzya" => "っじゃ",
  "zzyi" => "っじぃ",
  "zzyu" => "っじゅ",
  "zzye" => "っじぇ",
  "zzyo" => "っじょ",
  "da" => "だ",
  "di" => "ぢ",
  "du" => "づ",
  "de" => "で",
  "do" => "ど",
  "dya" => "ぢゃ",
  "dyi" => "ぢぃ",
  "dyu" => "ぢゅ",
  "dye" => "ぢぇ",
  "dyo" => "ぢょ",
  "dha" => "でゃ",
  "dhi" => "でぃ",
  "dhu" => "でゅ",
  "dhe" => "でぇ",
  "dho" => "でょ",
  "dwa" => "どぁ",
  "dwi" => "どぃ",
  "dwu" => "どぅ",
  "dwe" => "どぇ",
  "dwo" => "どぉ",
  "dda" => "っだ",
  "ddi" => "っぢ",
  "ddu" => "っづ",
  "dde" => "っで",
  "ddo" => "っど",
  "ddya" => "っぢゃ",
  "ddyi" => "っぢぃ",
  "ddyu" => "っぢゅ",
  "ddye" => "っぢぇ",
  "ddyo" => "っぢょ",
  "ddha" => "っでゃ",
  "ddhi" => "っでぃ",
  "ddhu" => "っでゅ",
  "ddhe" => "っでぇ",
  "ddho" => "っでょ",
  "ddwa" => "っどぁ",
  "ddwi" => "っどぃ",
  "ddwu" => "っどぅ",
  "ddwe" => "っどぇ",
  "ddwo" => "っどぉ",
  "ba" => "ば",
  "bi" => "び",
  "bu" => "ぶ",
  "be" => "べ",
  "bo" => "ぼ",
  "bya" => "びゃ",
  "byi" => "びぃ",
  "byu" => "びゅ",
  "bye" => "びぇ",
  "byo" => "びょ",
  "bba" => "っば",
  "bbi" => "っび",
  "bbu" => "っぶ",
  "bbe" => "っべ",
  "bbo" => "っぼ",
  "bbya" => "っびゃ",
  "bbyi" => "っびぃ",
  "bbyu" => "っびゅ",
  "bbye" => "っびぇ",
  "bbyo" => "っびょ",
  "pa" => "ぱ",
  "pi" => "ぴ",
  "pu" => "ぷ",
  "pe" => "ぺ",
  "po" => "ぽ",
  "pya" => "ぴゃ",
  "pyi" => "ぴぃ",
  "pyu" => "ぴゅ",
  "pye" => "ぴぇ",
  "pyo" => "ぴょ",
  "ppa" => "っぱ",
  "ppi" => "っぴ",
  "ppu" => "っぷ",
  "ppe" => "っぺ",
  "ppo" => "っぽ",
  "ppya" => "っぴゃ",
  "ppyi" => "っぴぃ",
  "ppyu" => "っぴゅ",
  "ppye" => "っぴぇ",
  "ppyo" => "っぴょ",
  "va" => "ゔぁ",
  "vi" => "ゔぃ",
  "vu" => "ゔ",
  "ve" => "ゔぇ",
  "vo" => "ゔぉ",
  "vya" => "ゔゃ",
  "vyi" => "ゔぃ",
  "vyu" => "ゔゅ",
  "vye" => "ゔぇ",
  "vyo" => "ゔょ",
  "vva" => "っゔぁ",
  "vvi" => "っゔぃ",
  "vvu" => "っゔ",
  "vve" => "っゔぇ",
  "vvo" => "っゔぉ",
  "vvya" => "っゔゃ",
  "vvyi" => "っゔぃ",
  "vvyu" => "っゔゅ",
  "vvye" => "っゔぇ",
  "vvyo" => "っゔょ",
  "qya" => "くゃ",
  "qyi" => "くぃ",
  "qyu" => "くゅ",
  "qye" => "くぇ",
  "qyo" => "くょ",
  "qwa" => "くぁ",
  "qwi" => "くぃ",
  "qwu" => "くぅ",
  "qwe" => "くぇ",
  "qwo" => "くぉ",
  "qa" => "くぁ",
  "qi" => "くぃ",
  "qu" => "くぅ",
  "qe" => "くぇ",
  "qo" => "くぉ",
  "qqya" => "っくゃ",
  "qqyi" => "っくぃ",
  "qqyu" => "っくゅ",
  "qqye" => "っくぇ",
  "qqyo" => "っくょ",
  "qqwa" => "っくぁ",
  "qqwi" => "っくぃ",
  "qqwu" => "っくぅ",
  "qqwe" => "っくぇ",
  "qqwo" => "っくぉ",
  "qqa" => "っくぁ",
  "qqi" => "っくぃ",
  "qqu" => "っくぅ",
  "qqe" => "っくぇ",
  "qqo" => "っくぉ",
  "fya" => "ふゃ",
  "fyi" => "ふぃ",
  "fyu" => "ふゅ",
  "fye" => "ふぇ",
  "fyo" => "ふょ",
  "fwa" => "ふぁ",
  "fwi" => "ふぃ",
  "fwu" => "ふぅ",
  "fwe" => "ふぇ",
  "fwo" => "ふぉ",
  "fa" => "ふぁ",
  "fi" => "ふぃ",
  "fu" => "ふ",
  "fe" => "ふぇ",
  "fo" => "ふぉ",
  "ffya" => "っふゃ",
  "ffyi" => "っふぃ",
  "ffyu" => "っふゅ",
  "ffye" => "っふぇ",
  "ffyo" => "っふょ",
  "ffwa" => "っふぁ",
  "ffwi" => "っふぃ",
  "ffwu" => "っふぅ",
  "ffwe" => "っふぇ",
  "ffwo" => "っふぉ",
  "ffa" => "っふぁ",
  "ffi" => "っふぃ",
  "ffu" => "っふ",
  "ffe" => "っふぇ",
  "ffo" => "っふぉ",
  "." => "。",
  "," => "、",
  ":" => "：",
  "/" => "・",
  "!" => "！",
  "?" => "？",
  "~" => "〜",
  "-" => "ー",
  "‘" => "「",
  "’" => "」",
  "“" => "『",
  "”" => "』",
  "[" => "［",
  "]" => "］",
  "(" => "（",
  ")" => "）",
  "{" => "｛",
  "}" => "｝",
  "xn" => "ん",
  "xtu" => "っ",
  "xtsu" => "っ",
  "xwa" => "ゎ",
  "xka" => "ヵ",
  "xke" => "ヶ",
  "xca" => "ヵ",
  "xce" => "ヶ",
  "xa" => "ぁ",
  "xi" => "ぃ",
  "xu" => "ぅ",
  "xe" => "ぇ",
  "xo" => "ぉ",
  "xya" => "ゃ",
  "xyi" => "ぃ",
  "xyu" => "ゅ",
  "xye" => "ぇ",
  "xyo" => "ょ",
  "ca" => "か",
  "ci" => "き",
  "cu" => "く",
  "ce" => "け",
  "co" => "こ",
  "cya" => "ちゃ",
  "cyi" => "ちぃ",
  "cyu" => "ちゅ",
  "cye" => "ちぇ",
  "cyo" => "ちょ",
  "cha" => "ちゃ",
  "chi" => "ち",
  "chu" => "ちゅ",
  "che" => "ちぇ",
  "cho" => "ちょ",
  "chya" => "ちゃ",
  "chyi" => "ちぃ",
  "chyu" => "ちゅ",
  "chye" => "ちぇ",
  "chyo" => "ちょ",
  "cca" => "っか",
  "cci" => "っき",
  "ccu" => "っく",
  "cce" => "っけ",
  "cco" => "っこ",
  "ccya" => "っちゃ",
  "ccyi" => "っちぃ",
  "ccyu" => "っちゅ",
  "ccye" => "っちぇ",
  "ccyo" => "っちょ",
  "ccha" => "っちゃ",
  "cchi" => "っち",
  "cchu" => "っちゅ",
  "cche" => "っちぇ",
  "ccho" => "っちょ",
  "cchya" => "っちゃ",
  "cchyi" => "っちぃ",
  "cchyu" => "っちゅ",
  "cchye" => "っちぇ",
  "cchyo" => "っちょ",
  "ja" => "じゃ",
  "ji" => "じ",
  "ju" => "じゅ",
  "je" => "じぇ",
  "jo" => "じょ",
  "jya" => "じゃ",
  "jyi" => "じぃ",
  "jyu" => "じゅ",
  "jye" => "じぇ",
  "jyo" => "じょ",
  "jja" => "っじゃ",
  "jji" => "っじ",
  "jju" => "っじゅ",
  "jje" => "っじぇ",
  "jjo" => "っじょ",
  "jjya" => "っじゃ",
  "jjyi" => "っじぃ",
  "jjyu" => "っじゅ",
  "jjye" => "っじぇ",
  "jjyo" => "っじょ",
  "ltu" => "っ",
  "ltsu" => "っ",
  "lwa" => "ゎ",
  "lka" => "ヵ",
  "lke" => "ヶ",
  "lca" => "ヵ",
  "lce" => "ヶ",
  "la" => "ぁ",
  "li" => "ぃ",
  "lu" => "ぅ",
  "le" => "ぇ",
  "lo" => "ぉ",
  "lya" => "ゃ",
  "lyi" => "ぃ",
  "lyu" => "ゅ",
  "lye" => "ぇ",
  "lyo" => "ょ"
};


}

