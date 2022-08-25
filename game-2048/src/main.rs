use bevy::prelude::*;

mod tile;
mod score;
mod states;
mod menu;
mod ai;
mod logic;

const BOARD: usize = 4;
const BACKGROUND_COLOR: Color = Color::rgb(250.0 / 255.0, 248.0 / 255.0, 239.0 / 255.0);

use tile::TilePlugin;
use states::GameState;
use menu::MenuPlugin;
use score::ScorePlugin;
use ai::AiPlugin;

const OFFSET: f32 = (-(4 as f32 / 2.0 * SQUARE_SIZE)) + SQUARE_SIZE / 2.;
const SQUARE_SIZE: f32 = 75.0;

fn main() {
    App::new()
    .insert_resource(ClearColor(BACKGROUND_COLOR))
    .insert_resource(WindowDescriptor {
        title: "2048".to_string(),
        width: 500.0,
        height: 500.0,
        ..default()
    })
    .add_state(GameState::Menu)
    .add_plugins(DefaultPlugins)
    .add_plugin(MenuPlugin)
    .add_plugin(ScorePlugin)
    .add_plugin(TilePlugin)  
    .add_plugin(AiPlugin)  
    .run();
}