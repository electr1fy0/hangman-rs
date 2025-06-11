use hangman;
use std::io;

fn compare_chars(data: &mut hangman::Data) -> bool {
    let wlen = data.length;

    // Take the guess from stdin
    println!("Enter your guess word.");
    println!("{} (Moves remaining: {})", data.bricks, data.moves);

    let mut guess = String::with_capacity(data.length);
    io::stdin()
        .read_line(&mut guess)
        .expect("Error reading guess");
    guess = guess.trim().to_string();

    let mut brick_vec: Vec<char> = data.bricks.chars().collect();
    let secret_vec: Vec<char> = data.secret.chars().collect();
    let guess_vec: Vec<char> = guess.chars().collect();

    for i in 0..wlen {
        if secret_vec[i] == guess_vec[i] {
            brick_vec[i] = secret_vec[i];
        }
    }
    data.bricks = brick_vec.into_iter().collect();
    data.moves -= 1;

    if data.bricks == data.secret {
        return true;
    }
    false
}

pub struct Game {
    pub score: u8,
    pub has_won: bool,
}

fn main() {
    println!("Welcome to Hangman! Let's hang.🧵");
    let mut data = hangman::config_data();

    loop {
        if compare_chars(&mut data) {
            println!("You won!");
            break;
        };

        if data.moves <= 0 {
            println!("You lose!");
            break;
        }
    }
}
