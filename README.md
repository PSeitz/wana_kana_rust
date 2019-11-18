 [![](http://meritbadge.herokuapp.com/wana_kana)](https://crates.io/crates/wana_kana)
 [![Docs](https://docs.rs/wana_kana/badge.svg)](https://docs.rs/crate/wana_kana/)
 [![Build Status](https://travis-ci.org/PSeitz/wana_kana_rust.svg?branch=master)](https://travis-ci.org/PSeitz/wana_kana_rust)
 [![Coverage Status](https://coveralls.io/repos/github/PSeitz/wana_kana_rust/badge.svg?branch=master)](https://coveralls.io/github/PSeitz/wana_kana_rust?branch=master)

 # WanaKana Rust
 ### ワナカナ <--> WanaKana <--> わなかな
```toml,ignore
[dependencies]
wana_kana = "2.0"
```


 Utility library for checking and converting between Japanese characters - Kanji, Hiragana, Katakana - and Romaji (Ported from https://github.com/WaniKani/WanaKana V4.0.2)
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

Version 2 Performane Improvements (Migrating from WanaKana 2.0.3 to 4.0.2)
```
 bench_hiragana_to_romaji    3,163            950                -2,213  -69.97%   x 3.33
 bench_kana_1                2,961            543                -2,418  -81.66%   x 5.45
 bench_kana_2                7,402            1,760              -5,642  -76.22%   x 4.21
 bench_katakana_to_hiragana  2,118            2,091                 -27   -1.27%   x 1.01
 bench_katakana_to_katakana  1,356            1,411                  55    4.06%   x 0.96
 bench_katakana_to_romaji    6,688            1,129              -5,559  -83.12%   x 5.92
 bench_romaji_to_hiragana    3,617            3,486                -131   -3.62%   x 1.04
 bench_romaji_to_katakana    4,167            1,882              -2,285  -54.84%   x 2.21
```

### CLI
#### Convert to kana and back for fun and profit
`cargo install wana_kana` will install 2 CLI tools: `to_kana` and `to_romaji`.

Both commands support piping `ls | to_kana` and parameters `to_romaji へろ　をるど`.
