// 2048 - Terminal, 12/03/2023
// (c) aichingert

use ruscii::app::{App, State};
use ruscii::gui::FPSCounter;
use ruscii::keyboard::{Key, KeyEvent};
use ruscii::terminal::{Window, Color};
use ruscii::drawing::{Pencil, RectCharset};
use ruscii::spatial::Vec2;

mod logic;
mod direction;

use logic::Game;
use direction::Direction;

const SIZE: usize = 4;
type Board = [[u16;SIZE];SIZE];

fn main() {
    let mut fps = FPSCounter::new();
    let mut app = App::new();

    let mut board: Board = [[0;SIZE];SIZE];
    let mut score: u16 = Game::start(&mut board);

    app.run(|app_state: &mut State, window: &mut Window| {
        for key_event in app_state.keyboard().last_key_events() {
            match key_event {
                KeyEvent::Pressed(Key::Esc) => app_state.stop(),
                KeyEvent::Pressed(Key::Q) => app_state.stop(),
                _ => (),
            }
        }

        fps.update();

        let mut pencil = Pencil::new(window.canvas_mut());
        pencil.draw_text(&format!("FPS: {}", fps.count()), Vec2::xy(1, 1));
        pencil.draw_text(&format!("Score: {}", score), Vec2::xy(10, 1));

        if app_state.step() & 1 == 0 {
            for key_down in app_state.keyboard().get_keys_down() {
                if let Some(new_score) = Game::update(&mut board, &match key_down {
                    Key::W => Direction::Up,
                    Key::S => Direction::Down,
                    Key::A => Direction::Left,
                    Key::D => Direction::Right, 
                    _ => Direction::Invalid,
                }) {
                    score = new_score;
                }             
            }
        }

        pencil.draw_rect(
            &RectCharset::simple_round_lines(),
            Vec2::xy(20,10),
            Vec2::xy(41,17)
        );

        for offset in 0..3 {
            pencil.draw_vline('\'', Vec2::xy(30 + offset*10,11), 15);
            pencil.draw_hline('\'', Vec2::xy(21, 14 + offset*4), 39);
        }

        for y in 0..board.len() {
            for x in 0..board.len() {
                if board[y][x] != 0 {
                    match board[y][x] {
                        2 => pencil.set_foreground(Color::Xterm(30)),
                        4 => pencil.set_foreground(Color::Xterm(33)),
                        8 => pencil.set_foreground(Color::Xterm(93)),
                        16 => pencil.set_foreground(Color::Xterm(34)),
                        32 => pencil.set_foreground(Color::Xterm(94)),
                        64 => pencil.set_foreground(Color::Xterm(36)),
                        128 => pencil.set_foreground(Color::Xterm(96)),
                        256 => pencil.set_foreground(Color::Xterm(32)),
                        512 => pencil.set_foreground(Color::Xterm(92)),
                        1024 => pencil.set_foreground(Color::Xterm(35)),
                        _ => pencil.set_foreground(Color::Xterm(95)),
                    };
                    pencil.draw_text(&format!("{}", board[y][x]), Vec2::xy(25 + x*10, 12 + y*4));
                }
            }
        }
    });
}
