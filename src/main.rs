use hangman::Data;
use hangman::Game;

use hangman::{self, compare_words, increment_score};
use std::io::{self};
fn main() {
    println!("Welcome to Hangman! Let's hang.🧵");

    let mut game = Game::new();

    loop {
        let mut data = Data::new();
        data.view_secret_if_debug();

        if compare_words(&mut data, &mut game) {
            println!("You won!");

            increment_score(&mut game.score);
            println!("Your score: {}", game.score);

            println!("Do you wanna continue playing?");
            let mut consent = String::new();
            io::stdin()
                .read_line(&mut consent)
                .expect("Error gaining consent😭");

            let consent = consent.trim();
            let consent = &consent.to_lowercase()[..];

            if consent == "yes" || consent == "y" {
                continue;
            }
            println!("Cool. See you later.");
            break;
        };

        if game.moves <= 0 {
            println!("You lose!");
            break;
        }
    }
}
