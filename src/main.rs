mod logic;
mod direction;

use std::io::{Write, stdin, stdout};

use logic::Game;
use direction::Direction;

const SIZE: usize = 4;
type Board = [[u16;SIZE];SIZE];

fn main() {
    let mut board: Board = [[0;SIZE];SIZE];
    let mut line: String = String::new();
    let mut score: u16 = Game::start(&mut board);

    loop {
        print!("\x1B[2J");
        Game::show(&board, score);
        line.clear();

        print!("Enter direction [Down(d), Up(u), Right(r), Left(l), Exit(x)]: ");
        stdout().flush().unwrap();

        stdin().read_line(&mut line).unwrap();
        let dir: Direction = Game::get_direction(line.trim());

        if line.trim().to_lowercase() == "x" {
            break;
        } else if dir == Direction::Invalid {
            continue;
        }

        if let Some(new) = Game::update(&mut board, &dir) {
            score = new;
        } else {
            println!("You lost: {score}");
            break;
        }
    }
}