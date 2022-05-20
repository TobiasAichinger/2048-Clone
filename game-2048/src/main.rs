use bevy::prelude::*;

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
    .add_plugins(DefaultPlugins)
    .add_startup_system(setup)
    .add_system(tile_system)    
    .run();
}

#[derive(Component)]
struct Square;

#[derive(Component, Debug, Clone)]
struct Tile {
    num: i32,
    pos: (i32, i32)
}

impl Tile {
    fn new(number: i32, position: (i32, i32)) -> Tile {
        Tile {
            num: number,
            pos: position,
        }
    }
}

fn tile_system(
    mut commands: Commands,
    mut query: Query<(&mut Transform, &Tile)>,
    keyboard_input: ResMut<Input<KeyCode>>,
    asset_server: Res<AssetServer>
) {
    let mut position_taken: bool = true;
    let mut first_check: bool = true;
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut test: Vec<Tile> = Vec::new();
    if keyboard_input.just_released(KeyCode::W) {
        while position_taken {
            position_taken = false;
            if !first_check {
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
            } else {
                first_check = false;
            }
    
            query.for_each( | (_, tile) | {
                if x == tile.pos.0 && y == tile.pos.1 {
                    position_taken = true;
                }
            });
        }
    
        let tile_position = Vec2::new(
            OFFSET + x as f32 * (SQUARE_SIZE),
            OFFSET + y as f32 * (SQUARE_SIZE),
        );    

        commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("two.png"),
            transform: Transform {
                translation: tile_position.extend(1.0),
                scale: Vec3::new(0.3, 0.3, 1.0),
                ..default()
            },
            ..default()
        })
        .insert(Tile::new(2, (x, y)));

        query.for_each_mut( | (tile_transform, to_clone) | {
            test.push(to_clone.clone());
    
            
            info!("{:?}", test);
        });
    }    
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
