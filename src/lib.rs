// Use rand if generating word instead of fetching
// use rand::prelude::*;
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
pub fn config_data() -> Data {
    // Read number of moves
    println!("Enter number of moves.");
    let mut moves: u8 = 0;
    u8_input(&mut moves);

    // Read input length
    println!("Enter word length.");
    let mut length = String::new();
    io::stdin()
        .read_line(&mut length)
        .expect("Error reading word length");
    let length: usize = length.trim().parse().expect("Error parsing word length");

    // Get a word from the API
    let secret = request_word(length).unwrap();
    // Make the secret visible if debug is set as an environment variable
    let visible = env::var("DEBUG").is_ok();
    if visible {
        dbg!(&secret);
    }

    // Construct the string of dashes
    let bricks = generate_bricks(length);

    let data = Data {
        length,
        moves,
        secret,
        bricks,
    };
    data
}

fn u8_input(num: &mut u8) {
    let mut string_var = String::new();

    io::stdin()
        .read_line(&mut string_var)
        .expect("Error reading string");

    *num = string_var.trim().parse().expect("Error parsing string");
}

pub fn generate_word(wlen: usize) -> String {
    let mut word = String::with_capacity(wlen);
    for _ in 0..word.capacity() {
        word.push(rand::random_range(b'a'..b'z') as char);
    }
    word
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_request() {
        let result = request_word(4).unwrap();
        assert!(result.len() == 4)
    }
}
