#[cfg(feature = "tokenize")]
use wana_kana::tokenize::*;

#[cfg(feature = "tokenize")]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenize_with_no_input() {
        let empty: Vec<String> = vec![];
        assert_eq!(tokenize(""), empty);
    }

    #[test]
    fn passes_basic_tests() {
        assert_eq!(tokenize("ふふ"), vec!["ふふ"]);
        assert_eq!(tokenize("フフ"), vec!["フフ"]);
        assert_eq!(tokenize("ふふフフ"), vec!["ふふ", "フフ"]);
        assert_eq!(tokenize("阮咸"), vec!["阮咸"]);
        assert_eq!(tokenize("感じ"), vec!["感", "じ"]);
        assert_eq!(tokenize("私は悲しい"), vec!["私", "は", "悲", "しい"]);
        assert_eq!(tokenize("ok لنذهب!"), vec!["ok", " ", "لنذهب", "!"]);
        assert_eq!(
            tokenize("what the...私は「悲しい」。"),
            vec!["what", " ", "the", "...", "私", "は", "「", "悲", "しい", "」。",]
        );
    }

    #[test]
    fn handles_mixed_input() {
        assert_eq!(
            tokenize("5romaji here...!?漢字ひらがなカタ　カナ４「ＳＨＩＯ」。！"),
            vec![
                "5",
                "romaji",
                " ",
                "here",
                "...!?",
                "漢字",
                "ひらがな",
                "カタ",
                "　",
                "カナ",
                "４",
                "「",
                "ＳＨＩＯ",
                "」。！",
            ]
        );
    }

    #[test]
    fn handles_mixed_input_compact_option() {
        assert_eq!(
            tokenize_with_opt(
                "5romaji here...!?漢字ひらがなカタ　カナ４「ＳＨＩＯ」。！ لنذهب",
                true
            ),
            vec![
                "5",
                "romaji here",
                "...!?",
                "漢字ひらがなカタ　カナ",
                "４「",
                "ＳＨＩＯ",
                "」。！",
                " ",
                "لنذهب",
            ]
        );
    }

    #[test]
    fn handles_mixed_input_detailed_option() {
        assert_eq!(
            tokenize_detailed(
                "5romaji here...!?漢字ひらがなカタ　カナ４「ＳＨＩＯ」。！ لنذهب",
                false
            ),
            vec![
                (TokenType::EnNum, "5".to_string()),
                (TokenType::En, "romaji".to_string()),
                (TokenType::Space, " ".to_string()),
                (TokenType::En, "here".to_string()),
                (TokenType::EnPunc, "...!?".to_string()),
                (TokenType::Kanji, "漢字".to_string()),
                (TokenType::Hiragana, "ひらがな".to_string()),
                (TokenType::Katakana, "カタ".to_string()),
                (TokenType::Space, "　".to_string()),
                (TokenType::Katakana, "カナ".to_string()),
                (TokenType::JaNum, "４".to_string()),
                (TokenType::JaPunc, "「".to_string()),
                (TokenType::Ja, "ＳＨＩＯ".to_string()),
                (TokenType::JaPunc, "」。！".to_string()),
                (TokenType::Space, " ".to_string()),
                (TokenType::Other, "لنذهب".to_string()),
            ]
        );
    }

    #[test]
    fn handles_mixed_input_detailed_and_compact_option() {
        assert_eq!(
            tokenize_detailed(
                "5romaji here...!?漢字ひらがなカタ　カナ４「ＳＨＩＯ」。！ لنذهب",
                true
            ),
            vec![
                (TokenType::Other, "5".to_string()),
                (TokenType::En, "romaji here".to_string()),
                (TokenType::Other, "...!?".to_string()),
                (TokenType::Ja, "漢字ひらがなカタ　カナ".to_string()),
                (TokenType::Other, "４「".to_string()),
                (TokenType::Ja, "ＳＨＩＯ".to_string()),
                (TokenType::Other, "」。！".to_string()),
                (TokenType::En, " ".to_string()),
                (TokenType::Other, "لنذهب".to_string()),
            ]
        );
    }
}
