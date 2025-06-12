use hangman::{self, compare_words};

pub struct Game {
    pub score: u8,
    pub has_won: bool,
}

fn main() {
    println!("Welcome to Hangman! Let's hang.🧵");

    let mut data = hangman::Data::new();
    data.view_secret_if_debug();

    loop {
        if compare_words(&mut data) {
            println!("You won!");
            break;
        };

        if data.moves <= 0 {
            println!("You lose!");
            break;
        }
    }
}
