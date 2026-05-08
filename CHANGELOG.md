5.0.0 (2026-05-08)
==================
### Breaking Changes
- [**breaking**] Add `Options::keep_prolonged_sound_mark` [#25](https://github.com/PSeitz/wana_kana_rust/pull/25) (thanks @exoego)
```
A new public field was added to `Options`. Code that constructs `Options`
with a complete struct literal must set `keep_prolonged_sound_mark`; using
`..Default::default()` continues to work.
```

### Features
- Add half-width katakana support [#21](https://github.com/PSeitz/wana_kana_rust/pull/21) (thanks @exoego)
```
Half-width katakana input is now normalized by `to_hiragana`, `to_katakana`,
and `to_romaji`, including dakuten/handakuten combinations, punctuation, and
the half-width prolonged sound mark. For example, `ﾜﾅｶﾅ` converts like `ワナカナ`.
```

- Add an option to keep the prolonged sound mark during hiragana conversion [#25](https://github.com/PSeitz/wana_kana_rust/pull/25) (thanks @exoego)
```
`Options::keep_prolonged_sound_mark` keeps chōonpu as `ー` instead of
expanding it to repeated vowels (`スーパー` -> `すーぱー` instead of `すうぱあ`).
Half-width `ｰ` is normalized to full-width `ー`.
```

### Documentation
- Promote `ConvertJapanese` in README examples [#18](https://github.com/PSeitz/wana_kana_rust/pull/18) (thanks @exoego)
- Document half-width katakana conversion examples in the README [#21](https://github.com/PSeitz/wana_kana_rust/pull/21) (thanks @exoego)
