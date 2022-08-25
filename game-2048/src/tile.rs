use bevy::prelude::*;
use rand::Rng;

use crate::states::GameState;
use crate::score::Score;
use crate::logic::Logic;

impl Plugin for TilePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                SystemSet::on_enter(GameState::Playing)
                    .with_system(setup)
            )
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(move_tiles)
                    .with_system(tile_system)
            );
    }
}

pub struct TilePlugin;

#[derive(Component)]
struct Square;

#[derive(Component, Debug, Clone, Copy)]
pub struct Tile {
    pub num: i32,
    pub pos: (usize, usize)
}

impl Tile {
    pub fn new(num: i32, pos: (usize, usize)) -> Tile {
        Tile {
            num,
            pos,
        }
    }
}

fn setup(
    mut commands: Commands,
) {
    for row in 0..super::BOARD {
        for col in 0..super::BOARD {
            let point_position = Vec2::new(
                super::OFFSET + col as f32 * (super::SQUARE_SIZE),
                super::OFFSET + row as f32 * (super::SQUARE_SIZE),
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

fn tile_system(
    mut commands: Commands,
    keyboard_input: ResMut<Input<KeyCode>>,
    materials: Res<AssetServer>,
    mut score_query: Query<&mut Score>,
    mut query: Query<(Entity, &mut Tile)>
) {
    if !keyboard_input.any_just_released([KeyCode::W, KeyCode::A, KeyCode::S, KeyCode::D, KeyCode::Down, KeyCode::Up, KeyCode::Left, KeyCode::Right]) {
        if query.is_empty() {
            spawn_random_tile(commands, materials, Logic::get_matrix(&vec![]));
        }

        return;
    }

    let new_tile: bool;
    let mut tiles: Vec<Tile> = vec![];
    query.iter().for_each( | (_, t) | tiles.push(*t));
    let mut matrix: [[Tile; super::BOARD]; super::BOARD];

    let dir: u8;

    if keyboard_input.just_released(KeyCode::Up) || keyboard_input.just_released(KeyCode::W) {
        dir = 0;
    } else if keyboard_input.just_released(KeyCode::Down) || keyboard_input.just_released(KeyCode::S) {
        dir = 1;
    } else if keyboard_input.just_released(KeyCode::Right) || keyboard_input.just_released(KeyCode::D) {
        dir = 2;
    } else {
        dir = 3;
    }

    matrix = Logic::get_matrix(&tiles);

    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            print!("{:?}", matrix[i][j].num);
        }
        println!();
    }

    println!();

    let matrix_clone: [[Tile; 4]; 4] = matrix.clone();

    for mut score in score_query.iter_mut() {
        score.0 += 1; // merge score later
    }

    new_tile = Logic::check_set_new_tile(matrix, matrix_clone);

    if new_tile {
        spawn_random_tile(commands, materials, matrix);
    }
}

fn move_tiles(
    time: Res<Time>, 
    mut query: Query<(&mut Transform, &Tile)>
) {
    for (mut transform, tile) in query.iter_mut() {
        // Get the direction to move in
        let direction = Vec3::new(super::OFFSET + tile.pos.0 as f32 * super::SQUARE_SIZE, super::OFFSET + (super::BOARD - 1 - tile.pos.1) as f32 * super::SQUARE_SIZE, 1.0) - transform.translation;
        // Only move if the piece isn't already there (distance is big)
        if direction.length() > 0.1 {
            transform.translation += direction.normalize() * (time.delta_seconds() * 500.);
        }
    }
}

fn spawn_random_tile(
    mut commands: Commands,
    materials: Res<AssetServer>,
    matrix: [[Tile; super::BOARD]; super::BOARD]
) {
    let mut positions: Vec<(usize, usize)> = Vec::new();
    let mut rng = rand::thread_rng();

    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            if matrix[i][j].num == 0 {
                positions.push((matrix[i][j].pos.0, matrix[i][j].pos.1));
            }
        }
    }
    
    if positions.len() > 0 {
        let idx: usize = rng.gen_range(0..positions.len());
        let tile_position = Vec2::new(
            super::OFFSET + positions[idx].0 as f32 * (super::SQUARE_SIZE),
            super::OFFSET + positions[idx].1 as f32 * (super::SQUARE_SIZE),
        );   

        let mut path: String;

        if rng.gen_range(0..10) == 9 {
            path = "4.png".to_string();
        } else {
            path = "2.png".to_string();
        }

        commands
        .spawn_bundle(SpriteBundle {
            texture: materials.load(&path),
            transform: Transform {
                translation: tile_position.extend(1.0),
                scale: Vec3::new(0.3, 0.3, 1.0),
                ..default()
            },
            ..default()
        })
        .insert(Tile::new(path.remove(0).to_digit(10).unwrap() as i32, (positions[idx].1, matrix.len() - 1 - positions[idx].1)));

        drop(rng);
    }
}