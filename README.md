 [![](http://meritbadge.herokuapp.com/wana_kana)](https://crates.io/crates/wana_kana)
 [![Docs](https://docs.rs/wana_kana/badge.svg)](https://docs.rs/crate/wana_kana/)
 [![Build Status](https://travis-ci.org/PSeitz/wana_kana_rust.svg?branch=master)](https://travis-ci.org/PSeitz/wana_kana_rust)
 [![Coverage Status](https://coveralls.io/repos/github/PSeitz/wana_kana_rust/badge.svg?branch=master)](https://coveralls.io/github/PSeitz/wana_kana_rust?branch=master)

 # WanaKana Rust
 ### ワナカナ <--> WanaKana <--> わなかな
```toml,ignore
[dependencies]
wana_kana = "0.9"
```


 Utility library for checking and converting between Japanese characters - Kanji, Hiragana, Katakana - and Romaji (Ported from https://github.com/WaniKani/WanaKana V2.3.4)
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
`cargo install wana_kana` will install 2 CLI tools: `to_kana` and `to_romaji`.

Both commands support piping `ls | to_kana` and parameters `to_romaji へろ　をるど`.
