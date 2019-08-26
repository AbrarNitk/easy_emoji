extern crate regex;
#[macro_use]
extern crate serde_derive;

extern crate serde_json;

use std::fs::{File};
use std::io::BufReader;
use std::io::BufRead;
use regex::Regex;

fn main() {
    println!("Hello, world! {}", "\u{1F1EE}\u{1F1F3}");
    read_json();
}

#[derive(Deserialize)]
pub struct EmojiS {
    pub emoji: String,
    pub aliases: Vec<String>,
}

fn read_json(){
    let default = "data/emoji.json";
    let default = File::open(default).expect("could not load default.json");
    let emojis: Vec<EmojiS> =
            serde_json::from_reader(default).expect("invalid json");
    for x in emojis.into_iter() {
        println!("{:?}, {}", x.aliases, x.emoji)
    }
    println!("{}", "\u{01f1fe}\u{01f1f9}")
}