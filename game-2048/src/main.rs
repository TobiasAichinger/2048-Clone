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
    .add_startup_system(setup)
    .add_state(GameState::Menu)
    .add_plugins(DefaultPlugins)
    .add_plugin(TilePlugin)    
    .run();
}

#[derive(Component)]
struct Square;
// Placing the 4 x 4 board on the screen
fn setup(
    mut commands: Commands,
) {
<<<<<<< HEAD
    for row in 0..4 {
        for col in 0..4 {
=======
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    for row in 0..BOARD {
        for col in 0..BOARD {
>>>>>>> 837fe2cf14c0f1f651aa5abf63ccee777901bb8b
            let point_position = Vec2::new(
                OFFSET + col as f32 * (SQUARE_SIZE),
                OFFSET + row as f32 * (SQUARE_SIZE),
            );

            commands
                .spawn()
                .insert(Square)
                .insert_bundle( SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgb(205.0 / 255.0, 193.0 / 255.0, 180.0 / 255.0),
                        ..default()
                    },
                    transform: Transform {
                        translation: point_position.extend(0.0),
                        scale: Vec3::new(50.0, 50.0, 1.0),
                        ..default()
                    },
                ..default()
            });
        } 
    }
}
