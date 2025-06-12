// //////////////////////////////////////////////////
// Use rand if generating word instead of fetching //
// use rand::prelude::*;                           //
// //////////////////////////////////////////////////
use reqwest;
use std::env;
use std::io;
use std::thread::panicking;

pub struct Data {
    pub secret: String,
    pub bricks: String,
    pub length: usize,
}

pub struct Game {
    pub score: u8,
    pub moves: u8,
}

impl Game {
    pub fn new() -> Self {
        println!("Enter number of moves.");
        let score = 0;
        let mut moves: u8 = 0;
        u8_input(&mut moves);

        Game { score, moves }
    }
    pub fn start(&mut self) {
        let mut data = Data::new();
        loop {
            data.view_secret_if_debug();
            if compare_words(self, &mut data) {
                println!("You won!");

                increment_score(&mut self.score);
                println!("Your score: {}", self.score);

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
            }
            if self.moves <= 0 {
                println!("You lose!");
                println!("The word was: {}", data.secret);
                break;
            }
        }
    }
}

impl Data {
    pub fn new() -> Data {
        println!("Enter word length.");
        let mut length = String::new();
        io::stdin()
            .read_line(&mut length)
            .expect("Error reading word length");

        let length: usize = length.trim().parse().expect("Error parsing word length");
        if length > 20 {
            panic!("Stay in your limits.")
        }
        let secret = request_word(length).expect("Error fetching the secret word");
        let bricks = generate_bricks(length);
        let data = Data {
            length,
            secret,
            bricks,
        };
        data
    }

    pub fn generate_word(wlen: usize) -> String {
        let mut word = String::with_capacity(wlen);
        for _ in 0..word.capacity() {
            word.push(rand::random_range(b'a'..b'z') as char);
        }
        word
    }

    pub fn view_secret_if_debug(&self) {
        let visible = env::var("DEBUG").is_ok();
        if visible {
            dbg!(&self.secret);
        }
    }
}

pub fn compare_words(game: &mut Game, data: &mut Data) -> bool {
    let wlen = data.length;

    println!("{} (Moves remaining: {})", data.bricks, game.moves);
    println!("Guess a letter.");

    let mut guess = String::with_capacity(2);
    io::stdin()
        .read_line(&mut guess)
        .expect("Error reading guess");
    let guess: char = guess.trim().parse().unwrap();

    let mut brick_vec: Vec<char> = data.bricks.chars().collect();
    let secret_vec: Vec<char> = data.secret.chars().collect();
    // let guess_vec: Vec<char> = guess.chars().collect();

    for i in 0..wlen {
        if secret_vec[i] == guess {
            brick_vec[i] = guess
        }
    }
    data.bricks = brick_vec.into_iter().collect();
    game.moves -= 1;

    if data.bricks == data.secret {
        return true;
    }
    false
}

pub fn increment_score(score: &mut u8) {
    *score += 1
}

pub fn request_word(wlen: usize) -> Result<String, Box<dyn std::error::Error>> {
    let url = format!(
        "https://random-word-api.vercel.app/api?words=1&length={}
",
        wlen
    );

    let res: Vec<String> = reqwest::blocking::get(url)?.json()?;
    Ok(res[0].clone())
}

pub fn generate_bricks(wlen: usize) -> String {
    let mut word = String::with_capacity(wlen);
    for _ in 0..wlen {
        word.push('-');
    }
    word
}

pub fn u8_input(num: &mut u8) {
    let mut string_var = String::new();

    io::stdin()
        .read_line(&mut string_var)
        .expect("Error reading string");

    *num = string_var.trim().parse().expect("Error parsing string");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_request() {
        let result = request_word(4).unwrap();
        assert!(result.len() == 4)
    }
}
