#![feature(slice_concat_ext)]
extern crate wana_kana;
use std::env;
use std::slice::SliceConcatExt;

use std::io::{self, Read};

fn main() {
    let args: String = env::args().skip(1).collect::<Vec<String>>().join(" ");
    if args.len() > 0 {
        println!("{}", wana_kana::to_kana::to_kana(&args));
    }

    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    if buffer.len() > 0 {
        println!("{}", wana_kana::to_kana::to_kana(&buffer));
    }
}