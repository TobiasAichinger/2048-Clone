mod logic;
mod direction;

use logic::Logic;
use direction::Direction;

const SIZE: usize = 4;
type Board = [[u16;SIZE];SIZE];

fn main() {
    let mut board: Board = [[0;SIZE];SIZE];
    let mut line: String = String::new();
    Logic::start(&mut board);

    loop {
        Logic::show(&board);
        line.clear();

        std::io::stdin().read_line(&mut line).unwrap();
        let dir: Direction = Logic::get_direction(line.trim());

        if dir == Direction::Invalid {
            println!("Direction is invalid: {}", &line);
            continue;
        }

        Logic::update(&mut board, dir);
    }
}