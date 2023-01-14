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
        Game::show(&board);
        line.clear();

        print!("[Score: {score}]Enter direction [Down(d), Up(u), Right(r), Left(l)]: ");
        stdout().flush().unwrap();

        stdin().read_line(&mut line).unwrap();
        let dir: Direction = Game::get_direction(line.trim());

        if dir == Direction::Invalid {
            println!("Direction is invalid: {}", &line);
            continue;
        }

        if let Some(new) = Game::update(&mut board, dir) {
            score = new;
        }
    }
}