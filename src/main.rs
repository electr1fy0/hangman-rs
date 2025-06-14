use std::io;

use hangman::Game;

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

        let mut table = Table::new();

        // let cross = Mark::Cross('X');
        // let circle = Mark::Circle('O');

        for i in 0..9 {
            print_table(&table);

            let player = if i & 1 == 1 { 'O' } else { 'X' };
            let index = get_input(player);

            table.data[index - 1] = player;

            let win_combinations = [
                // Rows
                [0, 1, 2],
                [3, 4, 5],
                [6, 7, 8],
                // Cols
                [0, 3, 6],
                [1, 4, 7],
                [2, 5, 8],
                // Diagonals
                [0, 4, 8],
                [2, 4, 6],
            ];

            for combo in win_combinations {
                let [a, b, c] = combo;
                if table.data[a] != '-'
                    && table.data[a] == table.data[b]
                    && table.data[b] == table.data[c]
                {
                    println!("{} won!", table.data[a]);
                    break;
                }
            }
        }
        println!("It's a draw!");
    } else {
        println!("Incorrect choice.");
    }
}

struct Table {
    data: [char; 9],
}
impl Table {
    fn new() -> Self {
        let mut arr: [char; 9] = ['u'; 9];
        let mut cnt: u8 = 1;
        for i in &mut arr {
            *i = (cnt + 48) as char;
            cnt += 1;
        }
        Table { data: arr }
    }
}
fn print_table(table: &Table) {
    let mut idx = 0;

    while idx < 9 {
        // if idx % 3 != 0 || idx == 0 {
        print!("{} ", table.data[idx]);

        if (idx + 1) % 3 == 0 && idx != 0 {
            println!("\n-----------");
        } else {
            print!("|  ")
        }
        idx += 1;
    }
}

// enum Mark {
//     Cross(char),
//     Circle(char),
// }

fn get_input(player: char) -> usize {
    println!("Enter index for {player} (1-9): ");
    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Error reading index");
    index.trim().parse::<usize>().expect("Error parsing input")
}
