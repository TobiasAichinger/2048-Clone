use bevy::prelude::*;

const BOARD_SIZE: usize = 4;

pub struct TilePlugin;

#[derive(Component, Debug, Clone, Copy)]
pub struct Tile {
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

impl Plugin for TilePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(tile_system);
    }
}


pub fn tile_system(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &mut Tile)>,
    keyboard_input: ResMut<Input<KeyCode>>,
    asset_server: Res<AssetServer>
) {
    // Variables for spawning tiles
    let mut position_taken: bool = true;
    let mut first_check: bool = true;
    let mut x: i32 = 0;
    let mut y: i32 = 0;

    // Variables for position and move detection
    let mut tiles: Vec<Tile> = Vec::new();
    let mut position_free: bool = true;
    let mut position_changed: bool = false;
    let mut transform_x: f32 = super::OFFSET;
    let mut transform_y: f32 = super::OFFSET;

    // Get all tiles on the board
    query.for_each( | ( _, _, tile) | {
        tiles.push(tile.clone());
    });

    // Move tiles up if W was pressed
    if keyboard_input.just_released(KeyCode::W) {
    }   
    
    // Move tiles down if S was pressed
    if keyboard_input.just_released(KeyCode::S) {
        query.for_each_mut( | (entity, mut transforming_tile, mut tile) | {
            position_changed = false;
            position_free = true;

            while position_free {
                for idx in 0..tiles.len() {
                    if tiles[idx].pos == tile.pos {
                        continue;
                    }


                    if tile.pos.1 + 1 == tiles[idx].pos.1 && tile.pos.0 == tiles[idx].pos.0 && tile.num == tiles[idx].num {
                        commands.entity(entity).despawn_recursive();
                    } else if tile.pos.1 - 1 == tiles[idx].pos.1 && tile.pos.0 == tiles[idx].pos.0
                    || tile.pos.1 - 1 <= -1 { // Check if one position above is free
                        position_free = false;
                    }
                }

                if position_free {
                    tile.pos.1 -= 1;
                    position_changed = true;
                }
            }

            if position_changed {
                transform_x += tile.pos.0 as f32 * super::SQUARE_SIZE;
                transform_y += tile.pos.1 as f32 * super::SQUARE_SIZE;
        
                transforming_tile.translation = Vec3::new(transform_x, transform_y, 1.0);
            }
        });
    } 

    // Move tiles right if D was pressed
    if keyboard_input.just_released(KeyCode::D) {
        query.for_each(|(entity, _, _)| {
            commands.entity(entity).despawn_recursive();
        });

        let mut matrix: [[Tile; BOARD_SIZE]; BOARD_SIZE] = get_matrix(&tiles);

        merge(&mut matrix, 3);

        for i in 0..matrix.len() {
            for j in 0..matrix[i].len() {
                if matrix[i][j].num != 0 {
                    let tile_position = Vec2::new(
                        super::OFFSET + matrix[i][j].pos.0 as f32 * (super::SQUARE_SIZE),
                        super::OFFSET + matrix[i][j].pos.1 as f32 * (super::SQUARE_SIZE),
                    );   
    
                    commands
                    .spawn_bundle(SpriteBundle {
                        texture: asset_server.load(&(matrix[i][j].num.to_string() + ".png")),
                        transform: Transform {
                            translation: tile_position.extend(1.0),
                            scale: Vec3::new(0.3, 0.3, 1.0),
                            ..default()
                        },
                        ..default()
                    })
                    .insert(Tile::new(matrix[i][j].num, (x, y)));
                }
            }
        }
    }

    // Move tiles left if A was pressed
    if keyboard_input.just_released(KeyCode::A) {
        query.for_each_mut( | (entity, mut transforming_tile, mut tile) | {
            position_changed = false;
            position_free = true;

            while position_free {
                for idx in 0..tiles.len() {
                    if tiles[idx].pos == tile.pos {
                        continue;
                    }

                    if tile.pos.1 + 1 == tiles[idx].pos.1 && tile.pos.0 == tiles[idx].pos.0 && tile.num == tiles[idx].num {
                        commands.entity(entity).despawn_recursive();
                    } else if tile.pos.0 - 1 == tiles[idx].pos.0 && tile.pos.1 == tiles[idx].pos.1
                    || tile.pos.0 - 1 <= -1 { // Check if one position above is free
                        position_free = false;
                    }
                }
    
                if position_free {
                    tile.pos.0 -= 1;
                    position_changed = true;
                }
            }
    
            if position_changed {
                transform_x += tile.pos.0 as f32 * super::SQUARE_SIZE;
                transform_y += tile.pos.1 as f32 * super::SQUARE_SIZE;
        
                transforming_tile.translation = Vec3::new(transform_x, transform_y, 1.0);
            }
        });
    }  

    if position_changed || tiles.len() == 0 {
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
    
            query.for_each( | (_, _, tile) | {
                if x == tile.pos.0 && y == tile.pos.1 {
                    position_taken = true;
                }
            });
        }
    
        let tile_position = Vec2::new(
            super::OFFSET + x as f32 * (super::SQUARE_SIZE),
            super::OFFSET + y as f32 * (super::SQUARE_SIZE),
        );    

        commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("2.png"),
            transform: Transform {
                translation: tile_position.extend(1.0),
                scale: Vec3::new(0.3, 0.3, 1.0),
                ..default()
            },
            ..default()
        })
        .insert(Tile::new(2, (x, y)));
    }

    if keyboard_input.just_released(KeyCode::I) {
        let mut arr = get_matrix(&tiles);

        merge(&mut arr, 0);

        for i in 0..arr.len() {
            for j in 0..arr[i].len() {
                print!("{:?}", arr[i][j]);
            }
            println!()
        }

        println!();
    }
}

fn get_matrix(tiles: &Vec<Tile>) -> [[Tile; BOARD_SIZE]; BOARD_SIZE] {
    let mut sorted: [[Tile; BOARD_SIZE]; BOARD_SIZE] = [[Tile::new(0, (0, 0)); BOARD_SIZE]; BOARD_SIZE];

    for tile in tiles {
        sorted[tile.pos.0 as usize][tile.pos.1 as usize] = *tile;
    }

    sorted
}

fn merge(tiles: &mut [[Tile; BOARD_SIZE]; BOARD_SIZE], direction: u8) {
    match direction {
        0 => {
        },
        1 => {

        },
        2 => {

        },
        _ => {
            let mut idx: usize = 0;

            // Push together

            for i in 1..tiles.len() {
                for j in 0..tiles[i].len() {
                    if tiles[i][j].num != 0 {
                        while tiles[i - (idx + 1)][j].num == 0 {
                            if idx < i - 1 {
                                idx += 1;
                            } else {
                                idx += 1;
                                break;
                            }
                        }

                        if i != i - idx {
                            tiles[i - idx][j].num = tiles[i][j].num;
                            tiles[i][j].num = 0;
                        }
                    }

                    idx = 0;
                }
            }

            // Merge everything next to each other together

            for i in 1..tiles.len() {
                for j in 0..tiles[i].len() {
                    if tiles[i][j].num != 0 && i != 0 {
                        if tiles[i][j].num == tiles[i - 1][j].num {
                            tiles[i - 1][j].num *= 2;
                            tiles[i][j].num = 0;
                        }
                    }
                } 
            }

            // Push everything together again

            for i in 1..tiles.len() {
                for j in 0..tiles[i].len() {
                    if tiles[i][j].num != 0 {
                        while tiles[i - (idx + 1)][j].num == 0 {
                            if idx < i - 1 {
                                idx += 1;
                            } else {
                                idx += 1;
                                break;
                            }
                        }

                        if i != i - idx {
                            tiles[i - idx][j].num = tiles[i][j].num;
                            tiles[i][j].num = 0;
                        }
                    }

                    idx = 0;
                }
            }

            set_position(tiles);
        }
    }
}

fn set_position(tiles: &mut [[Tile; BOARD_SIZE]; BOARD_SIZE]) {
    for i in 0..tiles.len() {
        for j in 0..tiles[i].len() {
            tiles[i][j].pos.0 = i as i32;
            tiles[i][j].pos.1 = j as i32;            
        }
    }
}