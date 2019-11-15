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

// pub fn to_kana_with_opt(orig: &str, options: Options) -> String {
//     // let config = options;
//     let input = &orig.to_lowercase();
//     let len = input.chars().count();
//     // Final output array
//     let mut ouput = String::with_capacity(input.len());
//     // Position in the string that is being evaluated
//     let mut cursor = 0;
//     let max_chunk = 4;

//     while cursor < len {
//         let mut chunk_size = std::cmp::min(max_chunk, len - cursor);
//         loop {
//             let chunk = get_chunk(&input, cursor, cursor + chunk_size);

//             let mapping = if options.use_obsolete_kana {
//                 TO_KANA_OBSOLETE.get(&chunk as &str)
//             }else if options.imemode{
//                 TO_KANA_IMEMODE.get(&chunk as &str)
//             }else{
//                 TO_KANA.get(&chunk as &str)
//             };

//             // let mapping = TO_KANA_FST.get(&chunk as &str).map(|num|TO_KANA_VALUES[num as usize]);

//             if let Some(mapping) = mapping {
//                 if get_chunk(&orig, cursor, cursor + chunk_size).chars().all(char::is_uppercase){
//                   ouput.push_str(&hiragana_to_katakana(mapping));
//                 }else{
//                   ouput.push_str(&mapping);
//                 }
//                 break;
//             }

//             chunk_size -= 1;
//             if chunk_size == 0 {
//                 ouput.push_str(&chunk);
//                 break;
//             }
//         };

//         cursor += std::cmp::max(chunk_size, 1);
//     }
//     ouput
// }

pub fn to_kana_with_opt(orig: &str, options: Options) -> String {
    // let config = options;
    let input = &orig.to_lowercase();
    let chars = input.chars().collect::<Vec<_>>();
    // Final output array
    let mut ouput = String::with_capacity(input.len());
    // Position in the string that is being evaluated
    // let _len = chars.len();


    let mut char_slice: &[char] = &chars;
    while char_slice.len() != 0  {

        let result = TO_NODE_TREE.get(&char_slice);
        //nothing find pass, through
        if result.1.len() == char_slice.len(){
            ouput.push(char_slice[0]);
            char_slice = &char_slice[1..];
        }else{
            ouput.push_str(result.0.unwrap());
            char_slice = result.1;
        }

    }

    ouput
}



#[derive(Debug)]
pub struct Node {
    pub transitions: Vec<(char, Node)>,
    pub output: Option<&'static str>
}

impl Node {
    fn get<'a>(&self, chars: &'a [char]) -> (Option<&'static str>, &'a [char]) {
        let mut i = 0;
        let mut curr_node = self;
        for char in chars.iter() {
            if let Some(trans_node) = curr_node.find_transition_node(*char) {
                curr_node = trans_node;
            }else{
                return (curr_node.output, &chars[i..])
            }
            i+=1;
        }

        if let Some(_output) = curr_node.output {
            (curr_node.output, &chars[i..])
        }else{
            (None, chars)
        }

    }

    fn find_transition_node<'a>(&self, char: char) -> Option<&Node> {
        // println!("{:?}", char);
        // println!("{:?}", self.transitions.iter().map(|t|&t.0).collect::<Vec<_>>());
        self.transitions.iter().find(|&t| t.0 == char).map(|t|&t.1)
    }

}


#[test]
fn test_node_tree() {
    let chars = ['a'];
    assert_eq!(TO_NODE_TREE.get(&chars).0, Some("あ"));
}

use fst::{Map, MapBuilder};
use fnv::FnvHashMap;
lazy_static! {

pub static ref TO_NODE_TREE: Node = {

    let transitions = vec![('!', Node { transitions: vec![], output: Some("！") }), ('(', Node { transitions: vec![], output: Some("（") }), (')', Node { transitions: vec![], output: Some("）") }), (',', Node { transitions: vec![], output: Some("、") }), ('-', Node { transitions: vec![], output: Some("ー") }), ('.', Node { transitions: vec![], output: Some("。") }), ('/', Node { transitions: vec![], output: Some("・") }), (':', Node { transitions: vec![], output: Some("：") }), ('?', Node { transitions: vec![], output: Some("？") }), ('[', Node { transitions: vec![], output: Some("［") }), (']', Node { transitions: vec![], output: Some("］") }), ('a', Node { transitions: vec![], output: Some("あ") }), ('b', Node { transitions: vec![('a', Node { transitions: vec![], output: Some("ば") }), ('b', Node { transitions: vec![('a', Node { transitions: vec![], output: Some("っば") }), ('e', Node { transitions: vec![], output: Some("っべ") }), ('i', Node { transitions: vec![], output: Some("っび") }), ('o', Node { transitions: vec![], output: Some("っぼ") }), ('u', Node { transitions: vec![], output: Some("っぶ") }), ('y', Node { transitions: vec![('a', Node { transitions: vec![], output: Some("っびゃ") }), ('e', Node { transitions: vec![], output: Some("っびぇ") }), ('i', Node { transitions: vec![], output: Some("っびぃ") }), ('o', Node { transitions: vec![], output: Some("っびょ") }), ('u', Node { transitions: vec![], output: Some("っびゅ") })], output: None })], output: None }), ('e', Node { transitions: vec![], output: Some("べ") }), ('i', Node { transitions: vec![], output: Some("び") }), ('o', Node { transitions: vec![], output: Some("ぼ") }), ('u', Node { transitions: vec![], output: Some("ぶ") }), ('y', Node { transitions: vec![('a', Node { transitions: vec![], output: Some("びゃ") }), ('e', Node { transitions: vec![], output: Some("びぇ") }), ('i', Node { transitions: vec![], output: Some("びぃ") }), ('o', Node { transitions: vec![], output: Some("びょ") }), ('u', Node { transitions: vec![], output: Some("びゅ") })], output: None })], output: None }), ('c', Node { transitions: vec![('a', Node { transitions: vec![], output: Some("か") }), ('c', Node { transitions: vec![('a', Node { transitions: vec![], output: Some("っか") }), ('e', Node { transitions: vec![], output: Some("っけ") }), ('h', Node { transitions: vec![('a', Node { transitions: vec![], output: Some("っちゃ") }), ('e', Node { transitions: vec![], output: Some("っちぇ") }), ('i', Node { transitions: vec![], output: Some("っち") }), ('o', Node { transitions: vec![], output: Some("っちょ") }), ('u', Node { transitions: vec![], output: Some("っちゅ") }), ('y', Node { transitions: vec![('a', Node { transitions: vec![], output: Some("っちゃ") }), ('e', Node { transitions: vec![], output: Some("っちぇ") }), ('i', Node { transitions: vec![], output: Some("っちぃ") }), ('o', Node { transitions: vec![], output: Some("っちょ") }), ('u', Node { transitions: vec![], output: Some("っちゅ") })], output: None })], output: None }), ('i', Node { transitions: vec![], output: Some("っき") }), ('o', Node { transitions: vec![], output: Some("っこ") }), ('u', Node { transitions: vec![], output: Some("っく") }), ('y', Node { transitions: vec![('a', Node { transitions: vec![], output: Some("っちゃ") }), ('e', Node { transitions: vec![], output: Some("っちぇ") }), ('i', Node { transitions: vec![], output: Some("っちぃ") }), ('o', Node { transitions: vec![], output: Some("っちょ") }), ('u', Node { transitions: vec![], output: Some("っちゅ") })], output: None })], output: None }), ('e', Node { transitions: vec![], output: Some("け") }), ('h', Node { transitions: vec![('a', Node { transitions: vec![], output: Some("ちゃ") }), ('e', Node { transitions: vec![], output: Some("ちぇ") }), ('i', Node { transitions: vec![], output: Some("ち") }), ('o', Node { transitions: vec![], output: Some("ちょ") }), ('u', Node { transitions: vec![], output: Some("ちゅ") }), ('y', Node { transitions: vec![('a', Node { transitions: vec![], output: Some("ちゃ") }), ('e', Node { transitions: vec![], output: Some("ちぇ") }), ('i', Node { transitions: vec![], output: Some("ちぃ") }), ('o', Node { transitions: vec![], output: Some("ちょ") }), ('u', Node { transitions: vec![], output: Some("ちゅ") })], output: None })], output: None }), ('i', Node { transitions: vec![], output: Some("き") }), ('o', Node { transitions: vec![], output: Some("こ") }), ('u', Node { transitions: vec![], output: Some("く") }), ('y', Node { transitions: vec![('a', Node { transitions: vec![], output: Some("ちゃ") }), ('e', Node { transitions: vec![], output: Some("ちぇ") }), ('i', Node { transitions: vec![], output: Some("ちぃ") }), ('o', Node { transitions: vec![], output: Some("ちょ") }), ('u', Node { transitions: vec![], output: Some("ちゅ") })], output: None })], output: None }), ('d', Node { transitions: vec![('a', Node { transitions: vec![], output: Some("だ") }), ('d', Node { transitions: vec![('a', Node { transitions: vec![], output: Some("っだ") }), ('e', Node { transitions: vec![], output: Some("っで") }), ('h', Node { transitions: vec![('a', Node { transitions: vec![], output: Some("っでゃ") }), ('e', Node { transitions: vec![], output: Some("っでぇ") }), ('i', Node { transitions: vec![], output: Some("っでぃ") }), ('o', Node { transitions: vec![], output: Some("っでょ") }), ('u', Node { transitions: vec![], output: Some("っでゅ") })], output: None }), ('i', Node { transitions: vec![], output: Some("っぢ") }), ('o', Node { transitions: vec![], output: Some("っど") }), ('u', Node { transitions: vec![], output: Some("っづ") }), ('w', Node { transitions: vec![('a', Node { transitions: vec![], output: Some("っどぁ") }), ('e', Node { transitions: vec![], output: Some("っどぇ") }), ('i', Node { transitions: vec![], output: Some("っどぃ") }), ('o', Node { transitions: vec![], output: Some("っどぉ") }), ('u', Node { transitions: vec![], output: Some("っどぅ") })], output: None }), ('y', Node { transitions: vec![('a', Node { transitions: vec![], output: Some("っぢゃ") }), ('e', Node { transitions: vec![], output: Some("っぢぇ") }), ('i', Node { transitions: vec![], output: Some("っぢぃ") }), ('o', Node { transitions: vec![], output: Some("っぢょ") }), ('u', Node { transitions: vec![], output: Some("っぢゅ") })], output: None })], output: None }), ('e', Node { transitions: vec![], output: Some("で") }), ('h', Node { transitions: vec![('a', Node { transitions: vec![], output: Some("でゃ") }), ('e', Node { transitions: vec![], output: Some("でぇ") }), ('i', Node { transitions: vec![], output: Some("でぃ") }), ('o', Node { transitions: vec![], output: Some("でょ") }), ('u', Node { transitions: vec![], output: Some("でゅ") })], output: None }), ('i', Node { transitions: vec![], output: Some("ぢ") }), ('o', Node { transitions: vec![], output: Some("ど") }), ('u', Node { transitions: vec![], output: Some("づ") }), ('w', Node { transitions: vec![('a', Node { transitions: vec![], output: Some("どぁ") }), ('e', Node { transitions: vec![], output: Some("どぇ") }), ('i', Node { transitions: vec![], output: Some("どぃ") }), ('o', Node { transitions: vec![], output: Some("どぉ") }), ('u', Node { transitions: vec![], output: Some("どぅ") })], output: None }), ('y', Node { transitions: vec![('a', Node { transitions: vec![], output: Some("ぢゃ") }), ('e', Node { transitions: vec![], output: Some("ぢぇ") }), ('i', Node { transitions: vec![], output: Some("ぢぃ") }), ('o', Node { transitions: vec![], output: Some("ぢょ") }), ('u', Node { transitions: vec![], output: Some("ぢゅ") })], output: None })], output: None }), ('e', Node { transitions: vec![], output: Some("え") }), ('f', Node { transitions: vec![('a', Node { transitions: vec![], output: Some("ふぁ") }), ('e', Node { transitions: vec![], output: Some("ふぇ") }), ('f', Node { transitions: vec![('a', Node { transitions: vec![], output: Some("っふぁ") }), ('e', Node { transitions: vec![], output: Some("っふぇ") }), ('i', Node { transitions: vec![], output: Some("っふぃ") }), ('o', Node { transitions: vec![], output: Some("っふぉ") }), ('u', Node { transitions: vec![], output: Some("っふ") }), ('w', Node { transitions: vec![('a', Node { transitions: vec![], output: Some("っふぁ") }), ('e', Node { transitions: vec![], output: Some("っふぇ") }), ('i', Node { transitions: vec![], output: Some("っふぃ") }), ('o', Node { transitions: vec![], output: Some("っふぉ") }), ('u', Node { transitions: vec![], output: Some("っふぅ") })], output: None }), ('y', Node { transitions: vec![('a', Node { transitions: vec![], output: Some("っふゃ") }), ('e', Node { transitions: vec![], output: Some("っふぇ") }), ('i', Node { transitions: vec![], output: Some("っふぃ") }), ('o', Node { transitions: vec![], output: Some("っふょ") }), ('u', Node { transitions: vec![], output: Some("っふゅ") })], output: None })], output: None }), ('i', Node { transitions: vec![], output: Some("ふぃ") }), ('o', Node { transitions: vec![], output: Some("ふぉ") }), ('u', Node { transitions: vec![], output: Some("ふ") }), ('w', Node { transitions: vec![('a', Node { transitions: vec![], output: Some("ふぁ") }), ('e', Node { transitions: vec![], output: Some("ふぇ") }), ('i', Node { transitions: vec![], output: Some("ふぃ") }), ('o', Node { transitions: vec![], output: Some("ふぉ") }), ('u', Node { transitions: vec![], output: Some("ふぅ") })], output: None }), ('y', Node { transitions: vec![('a', Node { transitions: vec![], output: Some("ふゃ") }), ('e', Node { transitions: vec![], output: Some("ふぇ") }), ('i', Node { transitions: vec![], output: Some("ふぃ") }), ('o', Node { transitions: vec![], output: Some("ふょ") }), ('u', Node { transitions: vec![], output: Some("ふゅ") })], output: None })], output: None }), ('g', Node { transitions: vec![('a', Node { transitions: vec![], output: Some("が") }), ('e', Node { transitions: vec![], output: Some("げ") }), ('g', Node { transitions: vec![('a', Node { transitions: vec![], output: Some("っが") }), ('e', Node { transitions: vec![], output: Some("っげ") }), ('i', Node { transitions: vec![], output: Some("っぎ") }), ('o', Node { transitions: vec![], output: Some("っご") }), ('u', Node { transitions: vec![], output: Some("っぐ") }), ('w', Node { transitions: vec![('a', Node { transitions: vec![], output: Some("っぐぁ") }), ('e', Node { transitions: vec![], output: Some("っぐぇ") }), ('i', Node { transitions: vec![], output: Some("っぐぃ") }), ('o', Node { transitions: vec![], output: Some("っぐぉ") }), ('u', Node { transitions: vec![], output: Some("っぐぅ") })], output: None }), ('y', Node { transitions: vec![('a', Node { transitions: vec![], output: Some("っぎゃ") }), ('e', Node { transitions: vec![], output: Some("っぎぇ") }), ('i', Node { transitions: vec![], output: Some("っぎぃ") }), ('o', Node { transitions: vec![], output: Some("っぎょ") }), ('u', Node { transitions: vec![], output: Some("っぎゅ") })], output: None })], output: None }), ('i', Node { transitions: vec![], output: Some("ぎ") }), ('o', Node { transitions: vec![], output: Some("ご") }), ('u', Node { transitions: vec![], output: Some("ぐ") }), ('w', Node { transitions: vec![('a', Node { transitions: vec![], output: Some("ぐぁ") }), ('e', Node { transitions: vec![], output: Some("ぐぇ") }), ('i', Node { transitions: vec![], output: Some("ぐぃ") }), ('o', Node { transitions: vec![], output: Some("ぐぉ") }), ('u', Node { transitions: vec![], output: Some("ぐぅ") })], output: None }), ('y', Node { transitions: vec![('a', Node { transitions: vec![], output: Some("ぎゃ") }), ('e', Node { transitions: vec![], output: Some("ぎぇ") }), ('i', Node { transitions: vec![], output: Some("ぎぃ") }), ('o', Node { transitions: vec![], output: Some("ぎょ") }), ('u', Node { transitions: vec![], output: Some("ぎゅ") })], output: None })], output: None }), ('h', Node { transitions: vec![('a', Node { transitions: vec![], output: Some("は") }), ('e', Node { transitions: vec![], output: Some("へ") }), ('h', Node { transitions: vec![('a', Node { transitions: vec![], output: Some("っは") }), ('e', Node { transitions: vec![], output: Some("っへ") }), ('i', Node { transitions: vec![], output: Some("っひ") }), ('o', Node { transitions: vec![], output: Some("っほ") }), ('u', Node { transitions: vec![], output: Some("っふ") }), ('y', Node { transitions: vec![('a', Node { transitions: vec![], output: Some("っひゃ") }), ('e', Node { transitions: vec![], output: Some("っひぇ") }), ('i', Node { transitions: vec![], output: Some("っひぃ") }), ('o', Node { transitions: vec![], output: Some("っひょ") }), ('u', Node { transitions: vec![], output: Some("っひゅ") })], output: None })], output: None }), ('i', Node { transitions: vec![], output: Some("ひ") }), ('o', Node { transitions: vec![], output: Some("ほ") }), ('u', Node { transitions: vec![], output: Some("ふ") }), ('y', Node { transitions: vec![('a', Node { transitions: vec![], output: Some("ひゃ") }), ('e', Node { transitions: vec![], output: Some("ひぇ") }), ('i', Node { transitions: vec![], output: Some("ひぃ") }), ('o', Node { transitions: vec![], output: Some("ひょ") }), ('u', Node { transitions: vec![], output: Some("ひゅ") })], output: None })], output: None }), ('i', Node { transitions: vec![], output: Some("い") }), ('j', Node { transitions: vec![('a', Node { transitions: vec![], output: Some("じゃ") }), ('e', Node { transitions: vec![], output: Some("じぇ") }), ('i', Node { transitions: vec![], output: Some("じ") }), ('j', Node { transitions: vec![('a', Node { transitions: vec![], output: Some("っじゃ") }), ('e', Node { transitions: vec![], output: Some("っじぇ") }), ('i', Node { transitions: vec![], output: Some("っじ") }), ('o', Node { transitions: vec![], output: Some("っじょ") }), ('u', Node { transitions: vec![], output: Some("っじゅ") }), ('y', Node { transitions: vec![('a', Node { transitions: vec![], output: Some("っじゃ") }), ('e', Node { transitions: vec![], output: Some("っじぇ") }), ('i', Node { transitions: vec![], output: Some("っじぃ") }), ('o', Node { transitions: vec![], output: Some("っじょ") }), ('u', Node { transitions: vec![], output: Some("っじゅ") })], output: None })], output: None }), ('o', Node { transitions: vec![], output: Some("じょ") }), ('u', Node { transitions: vec![], output: Some("じゅ") }), ('y', Node { transitions: vec![('a', Node { transitions: vec![], output: Some("じゃ") }), ('e', Node { transitions: vec![], output: Some("じぇ") }), ('i', Node { transitions: vec![], output: Some("じぃ") }), ('o', Node { transitions: vec![], output: Some("じょ") }), ('u', Node { transitions: vec![], output: Some("じゅ") })], output: None })], output: None }), ('k', Node { transitions: vec![('a', Node { transitions: vec![], output: Some("か") }), ('e', Node { transitions: vec![], output: Some("け") }), ('i', Node { transitions: vec![], output: Some("き") }), ('k', Node { transitions: vec![('a', Node { transitions: vec![], output: Some("っか") }), ('e', Node { transitions: vec![], output: Some("っけ") }), ('i', Node { transitions: vec![], output: Some("っき") }), ('o', Node { transitions: vec![], output: Some("っこ") }), ('u', Node { transitions: vec![], output: Some("っく") }), ('w', Node { transitions: vec![('a', Node { transitions: vec![], output: Some("っくぁ") })], output: None }), ('y', Node { transitions: vec![('a', Node { transitions: vec![], output: Some("っきゃ") }), ('e', Node { transitions: vec![], output: Some("っきぇ") }), ('i', Node { transitions: vec![], output: Some("っきぃ") }), ('o', Node { transitions: vec![], output: Some("っきょ") }), ('u', Node { transitions: vec![], output: Some("っきゅ") })], output: None })], output: None }), ('o', Node { transitions: vec![], output: Some("こ") }), ('u', Node { transitions: vec![], output: Some("く") }), ('w', Node { transitions: vec![('a', Node { transitions: vec![], output: Some("くぁ") })], output: None }), ('y', Node { transitions: vec![('a', Node { transitions: vec![], output: Some("きゃ") }), ('e', Node { transitions: vec![], output: Some("きぇ") }), ('i', Node { transitions: vec![], output: Some("きぃ") }), ('o', Node { transitions: vec![], output: Some("きょ") }), ('u', Node { transitions: vec![], output: Some("きゅ") })], output: None })], output: None }), ('l', Node { transitions: vec![('a', Node { transitions: vec![], output: Some("ぁ") }), ('c', Node { transitions: vec![('a', Node { transitions: vec![], output: Some("ヵ") }), ('e', Node { transitions: vec![], output: Some("ヶ") })], output: None }), ('e', Node { transitions: vec![], output: Some("ぇ") }), ('i', Node { transitions: vec![], output: Some("ぃ") }), ('k', Node { transitions: vec![('a', Node { transitions: vec![], output: Some("ヵ") }), ('e', Node { transitions: vec![], output: Some("ヶ") })], output: None }), ('o', Node { transitions: vec![], output: Some("ぉ") }), ('t', Node { transitions: vec![('s', Node { transitions: vec![('u', Node { transitions: vec![], output: Some("っ") })], output: None }), ('u', Node { transitions: vec![], output: Some("っ") })], output: None }), ('u', Node { transitions: vec![], output: Some("ぅ") }), ('w', Node { transitions: vec![('a', Node { transitions: vec![], output: Some("ゎ") })], output: None }), ('y', Node { transitions: vec![('a', Node { transitions: vec![], output: Some("ゃ") }), ('e', Node { transitions: vec![], output: Some("ぇ") }), ('i', Node { transitions: vec![], output: Some("ぃ") }), ('o', Node { transitions: vec![], output: Some("ょ") }), ('u', Node { transitions: vec![], output: Some("ゅ") })], output: None })], output: None }), ('m', Node { transitions: vec![('a', Node { transitions: vec![], output: Some("ま") }), ('e', Node { transitions: vec![], output: Some("め") }), ('i', Node { transitions: vec![], output: Some("み") }), ('m', Node { transitions: vec![('a', Node { transitions: vec![], output: Some("っま") }), ('e', Node { transitions: vec![], output: Some("っめ") }), ('i', Node { transitions: vec![], output: Some("っみ") }), ('o', Node { transitions: vec![], output: Some("っも") }), ('u', Node { transitions: vec![], output: Some("っむ") }), ('y', Node { transitions: vec![('a', Node { transitions: vec![], output: Some("っみゃ") }), ('e', Node { transitions: vec![], output: Some("っみぇ") }), ('i', Node { transitions: vec![], output: Some("っみぃ") }), ('o', Node { transitions: vec![], output: Some("っみょ") }), ('u', Node { transitions: vec![], output: Some("っみゅ") })], output: None })], output: None }), ('o', Node { transitions: vec![], output: Some("も") }), ('u', Node { transitions: vec![], output: Some("む") }), ('y', Node { transitions: vec![('a', Node { transitions: vec![], output: Some("みゃ") }), ('e', Node { transitions: vec![], output: Some("みぇ") }), ('i', Node { transitions: vec![], output: Some("みぃ") }), ('o', Node { transitions: vec![], output: Some("みょ") }), ('u', Node { transitions: vec![], output: Some("みゅ") })], output: None })], output: None }), ('n', Node { transitions: vec![('\'', Node { transitions: vec![], output: Some("ん") }), ('a', Node { transitions: vec![], output: Some("な") }), ('e', Node { transitions: vec![], output: Some("ね") }), ('i', Node { transitions: vec![], output: Some("に") }), ('o', Node { transitions: vec![], output: Some("の") }), ('u', Node { transitions: vec![], output: Some("ぬ") }), ('y', Node { transitions: vec![('a', Node { transitions: vec![], output: Some("にゃ") }), ('e', Node { transitions: vec![], output: Some("にぇ") }), ('i', Node { transitions: vec![], output: Some("にぃ") }), ('o', Node { transitions: vec![], output: Some("にょ") }), ('u', Node { transitions: vec![], output: Some("にゅ") })], output: None })], output: Some("ん") }), ('o', Node { transitions: vec![], output: Some("お") }), ('p', Node { transitions: vec![('a', Node { transitions: vec![], output: Some("ぱ") }), ('e', Node { transitions: vec![], output: Some("ぺ") }), ('i', Node { transitions: vec![], output: Some("ぴ") }), ('o', Node { transitions: vec![], output: Some("ぽ") }), ('p', Node { transitions: vec![('a', Node { transitions: vec![], output: Some("っぱ") }), ('e', Node { transitions: vec![], output: Some("っぺ") }), ('i', Node { transitions: vec![], output: Some("っぴ") }), ('o', Node { transitions: vec![], output: Some("っぽ") }), ('u', Node { transitions: vec![], output: Some("っぷ") }), ('y', Node { transitions: vec![('a', Node { transitions: vec![], output: Some("っぴゃ") }), ('e', Node { transitions: vec![], output: Some("っぴぇ") }), ('i', Node { transitions: vec![], output: Some("っぴぃ") }), ('o', Node { transitions: vec![], output: Some("っぴょ") }), ('u', Node { transitions: vec![], output: Some("っぴゅ") })], output: None })], output: None }), ('u', Node { transitions: vec![], output: Some("ぷ") }), ('y', Node { transitions: vec![('a', Node { transitions: vec![], output: Some("ぴゃ") }), ('e', Node { transitions: vec![], output: Some("ぴぇ") }), ('i', Node { transitions: vec![], output: Some("ぴぃ") }), ('o', Node { transitions: vec![], output: Some("ぴょ") }), ('u', Node { transitions: vec![], output: Some("ぴゅ") })], output: None })], output: None }), ('q', Node { transitions: vec![('a', Node { transitions: vec![], output: Some("くぁ") }), ('e', Node { transitions: vec![], output: Some("くぇ") }), ('i', Node { transitions: vec![], output: Some("くぃ") }), ('o', Node { transitions: vec![], output: Some("くぉ") }), ('q', Node { transitions: vec![('a', Node { transitions: vec![], output: Some("っくぁ") }), ('e', Node { transitions: vec![], output: Some("っくぇ") }), ('i', Node { transitions: vec![], output: Some("っくぃ") }), ('o', Node { transitions: vec![], output: Some("っくぉ") }), ('u', Node { transitions: vec![], output: Some("っくぅ") }), ('w', Node { transitions: vec![('a', Node { transitions: vec![], output: Some("っくぁ") }), ('e', Node { transitions: vec![], output: Some("っくぇ") }), ('i', Node { transitions: vec![], output: Some("っくぃ") }), ('o', Node { transitions: vec![], output: Some("っくぉ") }), ('u', Node { transitions: vec![], output: Some("っくぅ") })], output: None }), ('y', Node { transitions: vec![('a', Node { transitions: vec![], output: Some("っくゃ") }), ('e', Node { transitions: vec![], output: Some("っくぇ") }), ('i', Node { transitions: vec![], output: Some("っくぃ") }), ('o', Node { transitions: vec![], output: Some("っくょ") }), ('u', Node { transitions: vec![], output: Some("っくゅ") })], output: None })], output: None }), ('u', Node { transitions: vec![], output: Some("くぅ") }), ('w', Node { transitions: vec![('a', Node { transitions: vec![], output: Some("くぁ") }), ('e', Node { transitions: vec![], output: Some("くぇ") }), ('i', Node { transitions: vec![], output: Some("くぃ") }), ('o', Node { transitions: vec![], output: Some("くぉ") }), ('u', Node { transitions: vec![], output: Some("くぅ") })], output: None }), ('y', Node { transitions: vec![('a', Node { transitions: vec![], output: Some("くゃ") }), ('e', Node { transitions: vec![], output: Some("くぇ") }), ('i', Node { transitions: vec![], output: Some("くぃ") }), ('o', Node { transitions: vec![], output: Some("くょ") }), ('u', Node { transitions: vec![], output: Some("くゅ") })], output: None })], output: None }), ('r', Node { transitions: vec![('a', Node { transitions: vec![], output: Some("ら") }), ('e', Node { transitions: vec![], output: Some("れ") }), ('i', Node { transitions: vec![], output: Some("り") }), ('o', Node { transitions: vec![], output: Some("ろ") }), ('r', Node { transitions: vec![('a', Node { transitions: vec![], output: Some("っら") }), ('e', Node { transitions: vec![], output: Some("っれ") }), ('i', Node { transitions: vec![], output: Some("っり") }), ('o', Node { transitions: vec![], output: Some("っろ") }), ('u', Node { transitions: vec![], output: Some("っる") }), ('y', Node { transitions: vec![('a', Node { transitions: vec![], output: Some("っりゃ") }), ('e', Node { transitions: vec![], output: Some("っりぇ") }), ('i', Node { transitions: vec![], output: Some("っりぃ") }), ('o', Node { transitions: vec![], output: Some("っりょ") }), ('u', Node { transitions: vec![], output: Some("っりゅ") })], output: None })], output: None }), ('u', Node { transitions: vec![], output: Some("る") }), ('y', Node { transitions: vec![('a', Node { transitions: vec![], output: Some("りゃ") }), ('e', Node { transitions: vec![], output: Some("りぇ") }), ('i', Node { transitions: vec![], output: Some("りぃ") }), ('o', Node { transitions: vec![], output: Some("りょ") }), ('u', Node { transitions: vec![], output: Some("りゅ") })], output: None })], output: None }), ('s', Node { transitions: vec![('a', Node { transitions: vec![], output: Some("さ") }), ('e', Node { transitions: vec![], output: Some("せ") }), ('h', Node { transitions: vec![('a', Node { transitions: vec![], output: Some("しゃ") }), ('e', Node { transitions: vec![], output: Some("しぇ") }), ('i', Node { transitions: vec![], output: Some("し") }), ('o', Node { transitions: vec![], output: Some("しょ") }), ('u', Node { transitions: vec![], output: Some("しゅ") }), ('y', Node { transitions: vec![('a', Node { transitions: vec![], output: Some("しゃ") }), ('e', Node { transitions: vec![], output: Some("しぇ") }), ('i', Node { transitions: vec![], output: Some("しぃ") }), ('o', Node { transitions: vec![], output: Some("しょ") }), ('u', Node { transitions: vec![], output: Some("しゅ") })], output: None })], output: None }), ('i', Node { transitions: vec![], output: Some("し") }), ('o', Node { transitions: vec![], output: Some("そ") }), ('s', Node { transitions: vec![('a', Node { transitions: vec![], output: Some("っさ") }), ('e', Node { transitions: vec![], output: Some("っせ") }), ('h', Node { transitions: vec![('a', Node { transitions: vec![], output: Some("っしゃ") }), ('e', Node { transitions: vec![], output: Some("っしぇ") }), ('i', Node { transitions: vec![], output: Some("っし") }), ('o', Node { transitions: vec![], output: Some("っしょ") }), ('u', Node { transitions: vec![], output: Some("っしゅ") }), ('y', Node { transitions: vec![('a', Node { transitions: vec![], output: Some("っしゃ") }), ('e', Node { transitions: vec![], output: Some("っしぇ") }), ('i', Node { transitions: vec![], output: Some("っしぃ") }), ('o', Node { transitions: vec![], output: Some("っしょ") }), ('u', Node { transitions: vec![], output: Some("っしゅ") })], output: None })], output: None }), ('i', Node { transitions: vec![], output: Some("っし") }), ('o', Node { transitions: vec![], output: Some("っそ") }), ('u', Node { transitions: vec![], output: Some("っす") }), ('w', Node { transitions: vec![('a', Node { transitions: vec![], output: Some("っすぁ") }), ('e', Node { transitions: vec![], output: Some("っすぇ") }), ('i', Node { transitions: vec![], output: Some("っすぃ") }), ('o', Node { transitions: vec![], output: Some("っすぉ") }), ('u', Node { transitions: vec![], output: Some("っすぅ") })], output: None }), ('y', Node { transitions: vec![('a', Node { transitions: vec![], output: Some("っしゃ") }), ('e', Node { transitions: vec![], output: Some("っしぇ") }), ('i', Node { transitions: vec![], output: Some("っしぃ") }), ('o', Node { transitions: vec![], output: Some("っしょ") }), ('u', Node { transitions: vec![], output: Some("っしゅ") })], output: None })], output: None }), ('u', Node { transitions: vec![], output: Some("す") }), ('w', Node { transitions: vec![('a', Node { transitions: vec![], output: Some("すぁ") }), ('e', Node { transitions: vec![], output: Some("すぇ") }), ('i', Node { transitions: vec![], output: Some("すぃ") }), ('o', Node { transitions: vec![], output: Some("すぉ") }), ('u', Node { transitions: vec![], output: Some("すぅ") })], output: None }), ('y', Node { transitions: vec![('a', Node { transitions: vec![], output: Some("しゃ") }), ('e', Node { transitions: vec![], output: Some("しぇ") }), ('i', Node { transitions: vec![], output: Some("しぃ") }), ('o', Node { transitions: vec![], output: Some("しょ") }), ('u', Node { transitions: vec![], output: Some("しゅ") })], output: None })], output: None }), ('t', Node { transitions: vec![('a', Node { transitions: vec![], output: Some("た") }), ('e', Node { transitions: vec![], output: Some("て") }), ('h', Node { transitions: vec![('a', Node { transitions: vec![], output: Some("てゃ") }), ('e', Node { transitions: vec![], output: Some("てぇ") }), ('i', Node { transitions: vec![], output: Some("てぃ") }), ('o', Node { transitions: vec![], output: Some("てょ") }), ('u', Node { transitions: vec![], output: Some("てゅ") })], output: None }), ('i', Node { transitions: vec![], output: Some("ち") }), ('o', Node { transitions: vec![], output: Some("と") }), ('s', Node { transitions: vec![('a', Node { transitions: vec![], output: Some("つぁ") }), ('e', Node { transitions: vec![], output: Some("つぇ") }), ('i', Node { transitions: vec![], output: Some("つぃ") }), ('o', Node { transitions: vec![], output: Some("つぉ") }), ('u', Node { transitions: vec![], output: Some("つ") })], output: None }), ('t', Node { transitions: vec![('a', Node { transitions: vec![], output: Some("った") }), ('e', Node { transitions: vec![], output: Some("って") }), ('h', Node { transitions: vec![('a', Node { transitions: vec![], output: Some("ってゃ") }), ('e', Node { transitions: vec![], output: Some("ってぇ") }), ('i', Node { transitions: vec![], output: Some("ってぃ") }), ('o', Node { transitions: vec![], output: Some("ってょ") }), ('u', Node { transitions: vec![], output: Some("ってゅ") })], output: None }), ('i', Node { transitions: vec![], output: Some("っち") }), ('o', Node { transitions: vec![], output: Some("っと") }), ('s', Node { transitions: vec![('a', Node { transitions: vec![], output: Some("っつぁ") }), ('e', Node { transitions: vec![], output: Some("っつぇ") }), ('i', Node { transitions: vec![], output: Some("っつぃ") }), ('o', Node { transitions: vec![], output: Some("っつぉ") }), ('u', Node { transitions: vec![], output: Some("っつ") })], output: None }), ('u', Node { transitions: vec![], output: Some("っつ") }), ('w', Node { transitions: vec![('a', Node { transitions: vec![], output: Some("っとぁ") }), ('e', Node { transitions: vec![], output: Some("っとぇ") }), ('i', Node { transitions: vec![], output: Some("っとぃ") }), ('o', Node { transitions: vec![], output: Some("っとぉ") }), ('u', Node { transitions: vec![], output: Some("っとぅ") })], output: None }), ('y', Node { transitions: vec![('a', Node { transitions: vec![], output: Some("っちゃ") }), ('e', Node { transitions: vec![], output: Some("っちぇ") }), ('i', Node { transitions: vec![], output: Some("っちぃ") }), ('o', Node { transitions: vec![], output: Some("っちょ") }), ('u', Node { transitions: vec![], output: Some("っちゅ") })], output: None })], output: None }), ('u', Node { transitions: vec![], output: Some("つ") }), ('w', Node { transitions: vec![('a', Node { transitions: vec![], output: Some("とぁ") }), ('e', Node { transitions: vec![], output: Some("とぇ") }), ('i', Node { transitions: vec![], output: Some("とぃ") }), ('o', Node { transitions: vec![], output: Some("とぉ") }), ('u', Node { transitions: vec![], output: Some("とぅ") })], output: None }), ('y', Node { transitions: vec![('a', Node { transitions: vec![], output: Some("ちゃ") }), ('e', Node { transitions: vec![], output: Some("ちぇ") }), ('i', Node { transitions: vec![], output: Some("ちぃ") }), ('o', Node { transitions: vec![], output: Some("ちょ") }), ('u', Node { transitions: vec![], output: Some("ちゅ") })], output: None })], output: None }), ('u', Node { transitions: vec![], output: Some("う") }), ('v', Node { transitions: vec![('a', Node { transitions: vec![], output: Some("ゔぁ") }), ('e', Node { transitions: vec![], output: Some("ゔぇ") }), ('i', Node { transitions: vec![], output: Some("ゔぃ") }), ('o', Node { transitions: vec![], output: Some("ゔぉ") }), ('u', Node { transitions: vec![], output: Some("ゔ") }), ('v', Node { transitions: vec![('a', Node { transitions: vec![], output: Some("っゔぁ") }), ('e', Node { transitions: vec![], output: Some("っゔぇ") }), ('i', Node { transitions: vec![], output: Some("っゔぃ") }), ('o', Node { transitions: vec![], output: Some("っゔぉ") }), ('u', Node { transitions: vec![], output: Some("っゔ") }), ('y', Node { transitions: vec![('a', Node { transitions: vec![], output: Some("っゔゃ") }), ('e', Node { transitions: vec![], output: Some("っゔぇ") }), ('i', Node { transitions: vec![], output: Some("っゔぃ") }), ('o', Node { transitions: vec![], output: Some("っゔょ") }), ('u', Node { transitions: vec![], output: Some("っゔゅ") })], output: None })], output: None }), ('y', Node { transitions: vec![('a', Node { transitions: vec![], output: Some("ゔゃ") }), ('e', Node { transitions: vec![], output: Some("ゔぇ") }), ('i', Node { transitions: vec![], output: Some("ゔぃ") }), ('o', Node { transitions: vec![], output: Some("ゔょ") }), ('u', Node { transitions: vec![], output: Some("ゔゅ") })], output: None })], output: None }), ('w', Node { transitions: vec![('a', Node { transitions: vec![], output: Some("わ") }), ('e', Node { transitions: vec![], output: Some("うぇ") }), ('h', Node { transitions: vec![('a', Node { transitions: vec![], output: Some("うぁ") }), ('e', Node { transitions: vec![], output: Some("うぇ") }), ('i', Node { transitions: vec![], output: Some("うぃ") }), ('o', Node { transitions: vec![], output: Some("うぉ") }), ('u', Node { transitions: vec![], output: Some("う") })], output: None }), ('i', Node { transitions: vec![], output: Some("うぃ") }), ('o', Node { transitions: vec![], output: Some("を") }), ('u', Node { transitions: vec![], output: Some("う") }), ('w', Node { transitions: vec![('a', Node { transitions: vec![], output: Some("っわ") }), ('e', Node { transitions: vec![], output: Some("っうぇ") }), ('h', Node { transitions: vec![('a', Node { transitions: vec![], output: Some("っうぁ") }), ('e', Node { transitions: vec![], output: Some("っうぇ") }), ('i', Node { transitions: vec![], output: Some("っうぃ") }), ('o', Node { transitions: vec![], output: Some("っうぉ") }), ('u', Node { transitions: vec![], output: Some("っう") })], output: None }), ('i', Node { transitions: vec![], output: Some("っうぃ") }), ('o', Node { transitions: vec![], output: Some("っを") }), ('u', Node { transitions: vec![], output: Some("っう") })], output: None })], output: None }), ('x', Node { transitions: vec![('a', Node { transitions: vec![], output: Some("ぁ") }), ('c', Node { transitions: vec![('a', Node { transitions: vec![], output: Some("ヵ") }), ('e', Node { transitions: vec![], output: Some("ヶ") })], output: None }), ('e', Node { transitions: vec![], output: Some("ぇ") }), ('i', Node { transitions: vec![], output: Some("ぃ") }), ('k', Node { transitions: vec![('a', Node { transitions: vec![], output: Some("ヵ") }), ('e', Node { transitions: vec![], output: Some("ヶ") })], output: None }), ('n', Node { transitions: vec![], output: Some("ん") }), ('o', Node { transitions: vec![], output: Some("ぉ") }), ('t', Node { transitions: vec![('s', Node { transitions: vec![('u', Node { transitions: vec![], output: Some("っ") })], output: None }), ('u', Node { transitions: vec![], output: Some("っ") })], output: None }), ('u', Node { transitions: vec![], output: Some("ぅ") }), ('w', Node { transitions: vec![('a', Node { transitions: vec![], output: Some("ゎ") })], output: None }), ('y', Node { transitions: vec![('a', Node { transitions: vec![], output: Some("ゃ") }), ('e', Node { transitions: vec![], output: Some("ぇ") }), ('i', Node { transitions: vec![], output: Some("ぃ") }), ('o', Node { transitions: vec![], output: Some("ょ") }), ('u', Node { transitions: vec![], output: Some("ゅ") })], output: None })], output: None }), ('y', Node { transitions: vec![('a', Node { transitions: vec![], output: Some("や") }), ('e', Node { transitions: vec![], output: Some("いぇ") }), ('i', Node { transitions: vec![], output: Some("い") }), ('o', Node { transitions: vec![], output: Some("よ") }), ('u', Node { transitions: vec![], output: Some("ゆ") }), ('y', Node { transitions: vec![('a', Node { transitions: vec![], output: Some("っや") }), ('e', Node { transitions: vec![], output: Some("っいぇ") }), ('i', Node { transitions: vec![], output: Some("っい") }), ('o', Node { transitions: vec![], output: Some("っよ") }), ('u', Node { transitions: vec![], output: Some("っゆ") })], output: None })], output: None }), ('z', Node { transitions: vec![('a', Node { transitions: vec![], output: Some("ざ") }), ('e', Node { transitions: vec![], output: Some("ぜ") }), ('i', Node { transitions: vec![], output: Some("じ") }), ('o', Node { transitions: vec![], output: Some("ぞ") }), ('u', Node { transitions: vec![], output: Some("ず") }), ('y', Node { transitions: vec![('a', Node { transitions: vec![], output: Some("じゃ") }), ('e', Node { transitions: vec![], output: Some("じぇ") }), ('i', Node { transitions: vec![], output: Some("じぃ") }), ('o', Node { transitions: vec![], output: Some("じょ") }), ('u', Node { transitions: vec![], output: Some("じゅ") })], output: None }), ('z', Node { transitions: vec![('a', Node { transitions: vec![], output: Some("っざ") }), ('e', Node { transitions: vec![], output: Some("っぜ") }), ('i', Node { transitions: vec![], output: Some("っじ") }), ('o', Node { transitions: vec![], output: Some("っぞ") }), ('u', Node { transitions: vec![], output: Some("っず") }), ('y', Node { transitions: vec![('a', Node { transitions: vec![], output: Some("っじゃ") }), ('e', Node { transitions: vec![], output: Some("っじぇ") }), ('i', Node { transitions: vec![], output: Some("っじぃ") }), ('o', Node { transitions: vec![], output: Some("っじょ") }), ('u', Node { transitions: vec![], output: Some("っじゅ") })], output: None })], output: None })], output: None }), ('{', Node { transitions: vec![], output: Some("｛") }), ('}', Node { transitions: vec![], output: Some("｝") }), ('~', Node { transitions: vec![], output: Some("〜") }), ('‘', Node { transitions: vec![], output: Some("「") }), ('’', Node { transitions: vec![], output: Some("」") }), ('“', Node { transitions: vec![], output: Some("『") }), ('”', Node { transitions: vec![], output: Some("』") })];

    Node{transitions, output: None}
};

// pub static ref TO_NODE_TREE_OBSOLETE_KANA: Node = {

//     let tree = TO_NODE_TREE.clone();
//     tree.get["w"]
//     Node{transitions, output: None}
// };

pub static ref TO_KANA_OBSOLETE: FnvHashMap<&'static str, &'static str> = {
    let mut map: FnvHashMap<_, _> = TO_KANA.clone();
    map.insert("wi", "ゐ");
    map.insert("we", "ゑ");
    map
};

pub static ref TO_KANA_IMEMODE: FnvHashMap<&'static str, &'static str> = {
    let mut map: FnvHashMap<_, _> = TO_KANA.clone();
    map.remove("n");
    map.insert("nn", "ん");
    map.insert("n ", "ん");
    map
};

pub static ref TO_KANA_FST: Map = {
    let mut build = MapBuilder::memory();

    let mut vals: Vec<_> = TO_KANA.iter().collect();
    vals.sort();
    for (i, (k,_v)) in vals.iter().enumerate() {
        build.insert(k, i as u64).unwrap();
    }
    let bytes = build.into_inner().unwrap();
    let map = Map::from_bytes(bytes).unwrap();
    map
};

pub static ref TO_KANA_VALUES: Vec<&'static str> = {
    let mut vals: Vec<_> = TO_KANA.iter().collect();
    vals.sort();
    vals.into_iter().map(|(_k, v)| v).cloned().collect()
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

