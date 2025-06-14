use hangman::Game;

fn main() {
    println!("Welcome to Hangman! Let's hang.🧵");

    let mut game = Game::new();

    game.start();
}
