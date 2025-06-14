use std::io;

use hangman::Game;
// use module hangman;
mod tictactoe;

fn main() {
    println!("Pick a game!");
    println!("1. Hangman");
    println!("2. TicTacToe");

    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Error reading choice");
    let choice: u8 = choice.trim().parse().expect("Error parsing choice");
    play(choice);
}

fn play(choice: u8) {
    if choice == 1 {
        println!("Welcome to Hangman! Let's hang.🧵");

        let mut game = Game::new();

        game.start();
    } else if choice == 2 {
        println!("Welcome to TicTacToe! Let's start.🤞");

        tictactoe::logic();
    } else {
        println!("Incorrect choice.");
    }
}
