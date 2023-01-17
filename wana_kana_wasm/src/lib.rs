

use wasm_bindgen::prelude::*;
use wana_kana::to_romaji;
use wana_kana::to_kana;
use wana_kana::to_hiragana;
use wana_kana::to_katakana;

#[wasm_bindgen]
pub fn add(a: u32, b: u32) -> u32 {
    a + b
}

#[wasm_bindgen]
pub fn to_romaji(input: &str) -> String {
    to_romaji::to_romaji(input)
}

#[wasm_bindgen]
pub fn to_kana(input: &str) -> String {
    to_kana::to_kana(input)
}

#[wasm_bindgen]
pub fn to_hiragana(input: &str) -> String {
    to_hiragana::to_hiragana(input)
}

#[wasm_bindgen]
pub fn to_katakana(input: &str) -> String {
    to_katakana::to_katakana(input)
}