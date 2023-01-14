mod logic;
mod direction;

use logic::Logic;
use direction::Direction;

const SIZE: usize = 4;
type Board = [[u16;SIZE];SIZE];

fn main() {
    let mut board: Board = [[0;SIZE];SIZE];
    board[1][3] = 2;
    board[1][2] = 2;
    board[1][0] = 2;
    board[2][1] = 2;
    board[1][1] = 2;
    board[0][1] = 2;
    board[3][1] = 2;
    let mut line: String = String::new();

    loop {
        line.clear();
        std::io::stdin().read_line(&mut line).unwrap();
        let dir: Direction = Logic::get_direction(line.trim());

        if dir == Direction::Invalid {
            println!("Direction is invalid: {}", &line);
            continue;
        }

        Logic::update(&mut board, dir);
        for i in 0..board.len() {
            for j in 0..board.len() {
                print!("{:?}", board[i][j]);
            }
            println!();
        }
    }
}