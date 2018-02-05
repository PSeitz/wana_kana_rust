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
