//! Convert [Half-width Katakana](https://en.wikipedia.org/wiki/Half-width_kana). to [Hiragana](https://en.wikipedia.org/wiki/Hiragana)
//!
//! Passes through any non-half-width katakana chars
//!
//! # Examples
//!
//! halfwidth_katakana_to_hiragana('ｶﾀｶﾅ')
//!
//! // => "かたかな"
//!
//! halfwidth_katakana_to_hiragana('カタカナ is a type of kana')
//!
//! // => "カタカナ is a type of kana"

use crate::halfwidth_to_hiragana_node_tree::HALFWIDTH_KATAKANA_TO_HIRAGANA_NODE_TREE;

pub fn halfwidth_katakana_to_hiragana(orig: &str) -> String {
    let chars = orig.chars().collect::<Vec<_>>();
    let mut output = String::with_capacity(orig.len());
    let len = chars.len();
    // Position in the string that is being evaluated
    let mut curr_pos = 0;

    while curr_pos != len {
        let result = HALFWIDTH_KATAKANA_TO_HIRAGANA_NODE_TREE.get(&chars[curr_pos..]);
        // nothing found, pass through
        if result.1 == 0 {
            output.push(chars[curr_pos]);
            curr_pos += 1;
        } else {
            output.push_str(result.0);
            curr_pos += result.1;
        }
    }

    output
}

#[test]
fn test_halfwidth_katakana_to_hiragana() {
    assert_eq!(halfwidth_katakana_to_hiragana("ｶﾀｶﾅ"), "かたかな");
    assert_eq!(halfwidth_katakana_to_hiragana("カタカナ"), "カタカナ");
    assert_eq!(
        halfwidth_katakana_to_hiragana("カタカナ ｶﾀｶﾅ　is a type of kana"),
        "カタカナ かたかな is a type of kana"
    );
}
