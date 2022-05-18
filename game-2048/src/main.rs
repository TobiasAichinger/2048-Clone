use bevy::prelude::*;
use rand::prelude::*;

const OFFSET: f32 = (-(5 as f32 / 2.0 * SQUARE_SIZE)) + SQUARE_SIZE / 2.;

const SQUARE_SIZE: f32 = 50.0;

fn main() {
    App::new()
    .insert_resource(WindowDescriptor {
        title: "2048".to_string(),
        width: 500.0,
        height: 500.0,
        ..default()
    })
    .add_plugins(DefaultPlugins)
    .add_startup_system(setup)
    .run();
}

#[derive(Component)]
struct Square;

#[derive(Component)]
struct tile {
    size: i32,
    position: (i32, i32)
}

fn spawn_tiles(
    mut commands: Commands,
    
) {
    let x: i32 = 0;
}

fn setup(
    mut commands: Commands,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());


    for row in 0..4 {
        for col in 0..4 {
            let point_position = Vec2::new(
                OFFSET + col as f32 * (SQUARE_SIZE),
                OFFSET + row as f32 * (SQUARE_SIZE),
            );

            commands
                .spawn()
                .insert(Square)
                .insert_bundle( SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgb(255.0, 0.0, 0.0),
                        ..default()
                    },
                    transform: Transform {
                        translation: point_position.extend(0.0),
                        scale: Vec3::new(10.0, 10.0, 100.0),
                        ..default()
                    },
                ..default()
            });
        } 
    }
}
