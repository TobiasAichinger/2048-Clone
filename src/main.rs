// 2048 - Terminal, 15/01/2023
// (c) aichingert

use ruscii::app::{App, State};
use ruscii::gui::FPSCounter;
use ruscii::keyboard::{Key, KeyEvent};
use ruscii::terminal::Window;
use ruscii::drawing::{Pencil, RectCharset};
use ruscii::spatial::Vec2;

mod logic;
mod direction;

use std::io::{Write, stdin, stdout};

use logic::Game;
use direction::Direction;

const SIZE: usize = 4;
type Board = [[u16;SIZE];SIZE];

fn main() {
    let mut fps = FPSCounter::new();
    let mut app = App::new();
    let mut window_size = app.window().size();

    app.run(|app_state: &mut State, window: &mut Window| {
        for key_event in app_state.keyboard().last_key_events() {
            match key_event {
                KeyEvent::Pressed(Key::Esc) => app_state.stop(),
                KeyEvent::Pressed(Key::Q) => app_state.stop(),
                _ => (),
            }
        }

        for key_down in app_state.keyboard().get_keys_down() {
            match key_down {
                Key::W => (),
                Key::S => (),
                Key::A => (),
                Key::D => (),
                _ => (),
            }
        }

        fps.update();
        
        let mut pencil = Pencil::new(window.canvas_mut());
        pencil.draw_text(&format!("FPS: {}", fps.count()), Vec2::xy(1, 1));

        pencil.draw_rect(
            &RectCharset::simple_round_lines(),
            Vec2::xy(20,10),
            Vec2::xy(41,17)
        );

        for offset in 0..3 {
            pencil.draw_vline('\'', Vec2::xy(30 + offset*10,11), 15);
            pencil.draw_hline('\'', Vec2::xy(21, 14 + offset*4), 39);
        }
    });


    let mut board: Board = [[0;SIZE];SIZE];
    let mut line: String = String::new();
    let mut score: u16 = Game::start(&mut board);

    loop {
        // Clears the terminal
        print!("\x1B[2J");
        Game::show(&board, score);
        line.clear();

        // Gets the input from the user
        print!("Enter direction [Down(d), Up(u), Right(r), Left(l), Exit(x)]: ");
        stdout().flush().unwrap();

        stdin().read_line(&mut line).unwrap();
        let dir: Direction = Direction::from_string(line.trim());

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
