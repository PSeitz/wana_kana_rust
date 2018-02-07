 # WanaKana Rust

 ### ワナカナ <--> WanaKana <--> わなかな
 Utility library for checking and converting between Japanese characters - Kanji, Hiragana, Katakana - and Romaji (Ported from https://github.com/WaniKani/WanaKana)
 # Examples
 ```
 use wana_kana::to_romaji::*;
 use wana_kana::to_kana::*;
 use wana_kana::to_hiragana::*;
 use wana_kana::Options;
 assert_eq!(to_romaji("ワナカナ"), "wanakana");
 assert_eq!(to_hiragana("WanaKana"), "わなかな");
 assert_eq!(to_kana("WANAKANA"), "ワナカナ");
 ```

### CLI
#### Convert to kana and back for fun and profit
`cargo install wana_kana` will install 2 CLI tools: `to_kana` and `ro_romaji`.

Both support piping `ls | to_kana` and parameters `to_romaji へろ　をるど`
