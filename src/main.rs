use rand::prelude::*;
use std::env;
use std::io;
struct Word {
    data: String,
    length: usize,
    visible: bool,
}

fn generate_word(wlen: usize) -> String {
    let mut word = String::with_capacity(wlen);
    for _ in 0..word.capacity() {
        word.push(rand::random_range(b'a'..b'z') as char);
    }
    word
}

fn main() {
    println!("Welcome to Hangman! Let's hang.🧵");
    println!("Enter word length.");
    // parsing w-len logic
    let mut length = String::new();
    io::stdin()
        .read_line(&mut length)
        .expect("Error reading word length");
    let length: usize = length.trim().parse().expect("Error parsing word length");

    // word gen
    let data = generate_word(length);
    let visible = env::var("DEBUG").is_err();

    let word = Word {
        data,
        length,
        visible,
    };
}
