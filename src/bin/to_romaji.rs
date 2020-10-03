use std::env;

use std::io::{self, Read};

fn main() {
    let args: String = env::args().skip(1).collect::<Vec<String>>().join(" ");
    if !args.is_empty() {
        println!("{}", wana_kana::to_romaji::to_romaji(&args));
    } else {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer).unwrap();
        if !buffer.is_empty() {
            print!("{}", wana_kana::to_romaji::to_romaji(&buffer));
        }
    }
}
