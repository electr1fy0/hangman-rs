// //////////////////////////////////////////////////
// Use rand if generating word instead of fetching //
// use rand::prelude::*;                           //
// //////////////////////////////////////////////////
use reqwest;
use std::env;
use std::io;

pub struct Data {
    pub secret: String,
    pub bricks: String,
    pub length: usize,
    pub moves: u8,
}

impl Data {
    pub fn new() -> Data {
        println!("Enter number of moves.");
        let mut moves: u8 = 0;
        u8_input(&mut moves);

        println!("Enter word length.");
        let mut length = String::new();
        io::stdin()
            .read_line(&mut length)
            .expect("Error reading word length");

        let length: usize = length.trim().parse().expect("Error parsing word length");
        let secret = request_word(length).expect("Error fetching the secret word");
        let bricks = generate_bricks(length);

        let data = Data {
            length,
            moves,
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

pub fn compare_words(data: &mut Data) -> bool {
    let wlen = data.length;

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
