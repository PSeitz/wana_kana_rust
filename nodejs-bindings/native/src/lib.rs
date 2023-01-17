#[macro_use]
extern crate neon;

extern crate wana_kana;

use neon::prelude::*;
use wana_kana::to_romaji;
use wana_kana::to_kana;
use wana_kana::to_hiragana;
use wana_kana::to_katakana;


pub fn to_romaji(mut cx:FunctionContext) -> JsResult<JsString> {
	let result = to_romaji::to_romaji(&cx.argument::<JsString>(0)?.value());
    Ok(cx.string(result))
}

pub fn to_kana(mut cx:FunctionContext) -> JsResult<JsString> {
	let result = to_kana::to_kana(&cx.argument::<JsString>(0)?.value());
    Ok(cx.string(result))
}

pub fn to_hiragana(mut cx:FunctionContext) -> JsResult<JsString> {
	let result = to_hiragana::to_hiragana(&cx.argument::<JsString>(0)?.value());
    Ok(cx.string(result))
}

pub fn to_katakana(mut cx:FunctionContext) -> JsResult<JsString> {
	let result = to_katakana::to_katakana(&cx.argument::<JsString>(0)?.value());
    Ok(cx.string(result))
}

register_module!(mut cx, {
    cx.export_function("to_romaji", to_romaji)?;
    cx.export_function("to_kana", to_kana)?;
    cx.export_function("to_hiragana", to_hiragana)?;
    cx.export_function("to_katakana", to_katakana)?;
    Ok(())
});