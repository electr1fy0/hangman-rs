use rand::prelude::*;
use reqwest;
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
fn request_word(wlen: usize) -> Result<String, Box<dyn std::error::Error>> {
    let url = format!("https://random-word-api.herokuapp.com/word?length={}", wlen);
    let res = reqwest::blocking::get(url)?.text()?;

    Ok(res)
}

fn generate_bricks(wlen: usize) -> String {
    let mut word = String::with_capacity(wlen);
    for _ in 0..wlen {
        word.push('-');
    }
    word
}

fn compare_chars(key: &str, guess: &str, bricks: &str) {
    // [WIP]
    let wlen = guess.len();

    let mut chars: Vec<char> = bricks.chars().collect();

    for i in 0..wlen {
        if key.chars().nth(i) == guess.chars().nth(i) {
            println!("match");
            chars[i] = guess.chars().nth(i).unwrap();
        }
    }
}

fn main() {
    println!("Welcome to Hangman! Let's hang.🧵");

    // prasing moves
    println!("Enter number of moves.");
    let mut moves: u8 = 0;
    u8_input(&mut moves);
    println!("{moves}");
    println!("Enter word length.");
    // parsing w-len
    let mut length = String::new();
    io::stdin()
        .read_line(&mut length)
        .expect("Error reading word length");
    let length: usize = length.trim().parse().expect("Error parsing word length");

    // word gen
    let data = request_word(length).unwrap();
    let visible = env::var("DEBUG").is_ok();

    dbg!(visible);

    if visible {
        println!("{}", data);
    }
    let word = Word {
        data,
        length,
        visible,
    };

    let mut guess = String::with_capacity(word.length);
    io::stdin()
        .read_line(&mut guess)
        .expect("Error reading guess");
    dbg!(&guess);

    let mut bricks = generate_bricks(word.length);
    dbg!(&bricks);

    compare_chars(&word.data, &guess, &bricks);
}

fn u8_input(num: &mut u8) {
    let mut string_var = String::new();

    io::stdin()
        .read_line(&mut string_var)
        .expect("Error reading string");

    *num = string_var.trim().parse().expect("Error parsing string");
}
