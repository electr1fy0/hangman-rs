use hangman::Data;
use hangman::Game;

use hangman::{self, compare_words, increment_score};
use std::io::{self};

fn main() {
    println!("Welcome to Hangman! Let's hang.🧵");

    let mut game = Game::new();

    game.start();
}
