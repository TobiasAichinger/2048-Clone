use crate::direction::Direction;
use rand::prelude::*;

pub struct Game;

impl Game {
    pub fn start(board: &mut super::Board) -> u16 {
        Game::new(board);
        Game::update(board, Direction::Invalid).unwrap()
    }

    fn merge(board: &mut super::Board, dir: &Direction) {
        match dir {
            Direction::Up => {
                for i in 1..board.len() {
                    for j in 0..board.len() {
                        if board[i-1][j] == board[i][j] {
                            board[i-1][j] *= 2;
                            board[i][j] = 0;
                        }
                    }
                }
            },
            Direction::Down => {
                for i in 1..board.len() {
                    for j in 0..board.len() {
                        if board[board.len()-i][j] == board[board.len()-i-1][j] {
                            board[board.len()-i][j] *= 2;
                            board[board.len()-i-1][j] = 0;
                        }
                    }
                }
            },
            Direction::Left => {
                for i in 1..board.len() {
                    for j in 0..board.len() {
                        if board[j][i-1] == board[j][i] {
                            board[j][i-1] *= 2;
                            board[j][i] = 0;
                        }
                    }
                }
            },
            Direction::Right => {
                for i in 1..board.len() {
                    for j in 0..board.len() {
                        if board[j][board.len()-i] == board[j][board.len()-i-1] {
                            board[j][board.len()-i] *= 2;
                            board[j][board.len()-i-1] = 0;
                        }
                    }
                }
            },
            Direction::Invalid => (),
        }
    }

    fn push(board: &mut super::Board, dir: &Direction) {
        match dir {
            Direction::Up => {
                for i in 1..board.len() {
                    for j in 0..board.len() {
                        let mut y: usize = i;
                        while y > 0 {
                            if board[y-1][j] != 0 { break; }
                            board[y-1][j] = board[y][j];
                            board[y][j] = 0;
                            y -= 1;
                        }
                    }
                }
            },
            Direction::Down => {
                for i in 2..=board.len() {
                    for j in 0..board.len() {
                        let mut y: usize = board.len() - i;
                        while y < board.len()-1 {
                            if board[y+1][j] != 0 { break; }
                            board[y+1][j] = board[y][j];
                            board[y][j] = 0;
                            y += 1;
                        }
                    }
                }
            },
            Direction::Left => {
                for i in 1..board.len() {
                    for j in 0..board.len() {
                        let mut y: usize = i;
                        while y > 0 {
                            if board[j][y-1] != 0 { break; }
                            board[j][y-1] = board[j][y];
                            board[j][y] = 0;
                            y -= 1;
                        }
                    }
                }
            },
            Direction::Right => {
                for i in 2..=board.len() {
                    for j in 0..board.len() {
                        let mut y: usize = board.len() - i;
                        while y < board.len()-1 {
                            if board[j][y+1] != 0 { break; }
                            board[j][y+1] = board[j][y];
                            board[j][y] = 0;
                            y += 1;
                        }
                    }
                }
            },
            Direction::Invalid => (),
        }
    }

    fn new(board: &mut super::Board) {
        let mut open: Vec<(usize,usize)> = Vec::new();
        let mut rnd = rand::thread_rng();

        for i in 0..board.len() {
            for j in 0..board[i].len() {
                if board[i][j] == 0 { open.push((i,j)); }
            }
        }

        let new: u16 = match rnd.gen_range(0..10) {
            9 => 4,
            _ => 2,
        };

        open.shuffle(&mut rnd);
        board[open[0].0][open[0].1] = new;
    }

    pub fn update(board: &mut super::Board, dir: Direction) -> Option<u16> {
        let sum: u16 = board
            .iter()
            .map(|sl| 
                sl
                    .iter()
                    .map(|e| e)
                .sum::<u16>())
            .sum::<u16>();
        let clone = board.clone();

        Game::push(board, &dir);
        Game::merge(board, &dir);
        Game::push(board, &dir);

        for i in 0..clone.len() {
            if clone[i] != board[i] {
                Game::new(board);
                break;
            }
        }

        Some(sum)
    }

    pub fn get_direction(line: &str) -> Direction {
        match line.to_lowercase().as_str() {
            "up" | "u" => Direction::Up,
            "down" | "d" => Direction::Down,
            "left" | "l" => Direction::Left,
            "right" | "r" => Direction::Right,
            _ => Direction::Invalid,
        }
    }

    pub fn show(board: &super::Board) {
        for i in 0..board.len() {
            println!("\x1B[90m-----------------------------");
            print!("|");
            for j in 0..board.len() {
                match board[i][j] {
                    0 => print!("\x1B[30m{:>5} \x1B[90m|", board[i][j]),
                    2 => print!("\x1B[33m{:>5} \x1B[90m|", board[i][j]),
                    4 => print!("\x1B[93m{:>5} \x1B[90m|", board[i][j]),
                    8 => print!("\x1B[34m{:>5} \x1B[90m|", board[i][j]),
                    16 => print!("\x1B[94m{:>5} \x1B[90m|", board[i][j]),
                    32 => print!("\x1B[36m{:>5} \x1B[90m|", board[i][j]),
                    64 => print!("\x1B[96m{:>5} \x1B[90m|", board[i][j]),
                    128 => print!("\x1B[36m{:>5} \x1B[90m|", board[i][j]),
                    256 => print!("\x1B[96m{:>5} \x1B[90m|", board[i][j]),
                    512 => print!("\x1B[32m{:>5} \x1B[90m|", board[i][j]),
                    1024 => print!("\x1B[92m{:>5} \x1B[90m|", board[i][j]),
                    2048 => print!("\x1B[35m{:>5} \x1B[90m|", board[i][j]),
                    4096 => print!("\x1B[95m{:>5} \x1B[90m|", board[i][j]),
                    _ => panic!("Number too high!")
                };
            }
            println!();
        }
        println!("\x1B[90m-----------------------------");
        print!("\x1B[0m");
    }
}