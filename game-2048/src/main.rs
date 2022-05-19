use std::thread::sleep_ms;

use bevy::prelude::*;

const OFFSET: f32 = (-(5 as f32 / 2.0 * SQUARE_SIZE)) + SQUARE_SIZE / 2.;

const SQUARE_SIZE: f32 = 50.0;

fn main() {
    App::new()
    .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
    .insert_resource(WindowDescriptor {
        title: "2048".to_string(),
        width: 500.0,
        height: 500.0,
        ..default()
    })
    .add_plugins(DefaultPlugins)
    .add_startup_system(setup)
    .add_system(spawn_tiles)
    .run();
}

#[derive(Component)]
struct Square;

#[derive(Component)]
struct Tile {
    number: i32,
    position: (i32, i32)
}


fn spawn_tiles(
    mut commands: Commands,
    query: Query<&Tile>
) {
    let mut position_taken: bool = true;
    let mut x: i32 = 0;
    let mut y: i32 = 0;

    while position_taken {
        position_taken = false;

        query.for_each( | tile | {
            if x == tile.position.0 || y == tile.position.1 {
                info!("{:?}", tile.position);
                position_taken = true;
            }
        });

        if (x + 1) % 4 == 0 && y < 3 {
            y += 1;
            x = 0;
        } else if x < 3 {
            x += 1;
        }
        else {
            // Check if there are possible ways to move the tiles if not exit (game lost)
            break;
        }
    }

    let tile_position = Vec2::new(
        OFFSET + x as f32 * (SQUARE_SIZE),
        OFFSET + y as f32 * (SQUARE_SIZE),
    );    

    commands
        .spawn()
        .insert(Tile {
            number: 2,
            position: (x, y)
        })
        .insert_bundle( SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.0, 1.0, 0.0),
                ..default()
            },
            transform: Transform {
                translation: tile_position.extend(0.0),
                scale: Vec3::new(35.0, 35.0, 1.0),
                ..default()
            },
        ..default()
    });
}

/*
fn random_number(min: i32, max: i32) -> i32 {
    let mut rng = thread_rng;
    rng.gen_range(min..max)
}
*/

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
                        scale: Vec3::new(35.0, 35.0, 1.0),
                        ..default()
                    },
                ..default()
            });
        } 
    }
}
