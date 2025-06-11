use hangman;
use rand::prelude::*;
use reqwest;
use std::env;
use std::io;

// fn compare_chars(data: &mut Data) {
//     // [WIP]
//     let wlen = guess.len();

//     let mut chars: Vec<char> = bricks.chars().collect();

//     for i in 0..wlen {
//         if key.chars().nth(i) == guess.chars().nth(i) {
//             dbg!(&key.chars().nth(i));
//             chars[i] = guess.chars().nth(i).unwrap();
//         }
//     }
// }

fn main() {
    println!("Welcome to Hangman! Let's hang.🧵");
    let mut data = hangman::config_data();

    // compare_chars(&mut data);
}
