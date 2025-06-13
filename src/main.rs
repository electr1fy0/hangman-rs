use hangman::Game;

fn main() {
    println!("Welcome to Hangman! Let's hang.🧵");
    let mutiline = format!(
        "
=========
    +---+
    |   |
    O   |
   /|\\  |
   / \\  |
        |
=========\n\n"
    );

    let arts = [
        "
=========
    +---+
    |   |
    O   |
    /|\\  |
    / \\  |
        |
=========",
        "
=========
    +---+
    |   |
    O   |
    /|\\  |
          |
        |
=========",
        "
=========
    +---+
    |   |
    O   |
    /|    |
          |
        |
=========",
        "
=========
    +---+
    |   |
    O   |
         |
         |
        |
=========",
    ];
    print!("{}", mutiline);
    let mut game = Game::new();

    game.start();
}
