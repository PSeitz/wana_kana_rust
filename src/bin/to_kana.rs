use std::env;
use std::io::{self, Read};

use wana_kana::ConvertJapanese;

fn main() {
    let args: String = env::args().skip(1).collect::<Vec<String>>().join(" ");
    if !args.is_empty() {
        println!("{}", (&args.to_kana()));
    } else {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer).unwrap();
        if !buffer.is_empty() {
            print!("{}", (&buffer.to_kana()));
        }
    }
}
