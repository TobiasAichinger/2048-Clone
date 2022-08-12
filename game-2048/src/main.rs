use bevy::prelude::*;

mod tile;
mod score;
mod states;
mod menu;

const BOARD: usize = 4;

use tile::TilePlugin;
use states::GameState;

const OFFSET: f32 = (-(4 as f32 / 2.0 * SQUARE_SIZE)) + SQUARE_SIZE / 2.;
const SQUARE_SIZE: f32 = 75.0;

fn main() {
    App::new()
    .insert_resource(ClearColor(Color::rgb(250.0 / 255.0, 248.0 / 255.0, 239.0 / 255.0)))
    .insert_resource(WindowDescriptor {
        title: "2048".to_string(),
        width: 500.0,
        height: 500.0,
        ..default()
    })
    .add_state(GameState::Menu)
    .add_plugins(DefaultPlugins)
    .add_plugin(TilePlugin)    
    .run();
}