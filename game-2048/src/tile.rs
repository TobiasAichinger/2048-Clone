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
    query: Query<(Entity, &mut Transform, &mut Tile)>,
    keyboard_input: ResMut<Input<KeyCode>>,
    asset_server: Res<AssetServer>
) {
    // Variables for position and move detection
    let mut tiles: Vec<Tile> = Vec::new();

    // Get all tiles on the board
    query.for_each( | ( _, _, tile) | {
        tiles.push(tile.clone());
    });

    // Move tiles up if W was pressed
    if keyboard_input.just_released(KeyCode::W) {
        // Removes all entitys
        query.for_each(|(entity, _, _)| {
            commands.entity(entity).despawn_recursive();
        });

        let mut matrix: [[Tile; BOARD_SIZE]; BOARD_SIZE] = get_matrix(&tiles);

        merge(&mut matrix, 2);

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
                    .insert(Tile::new(matrix[i][j].num, (matrix[i][j].pos.1, matrix[i][j].pos.0)));
                }
            }
        }
    }   
    
    // Move tiles down if S was pressed
    if keyboard_input.just_released(KeyCode::S) {
        // Removes all entitys
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
                    .insert(Tile::new(matrix[i][j].num, (matrix[i][j].pos.1, matrix[i][j].pos.0)));
                }
            }
        }
    } 

    // Move tiles right if D was pressed
    if keyboard_input.just_released(KeyCode::D) {
        // Removes all entitys
        query.for_each(|(entity, _, _)| {
            commands.entity(entity).despawn_recursive();
        });

        let mut matrix: [[Tile; BOARD_SIZE]; BOARD_SIZE] = get_matrix(&tiles);

        merge(&mut matrix, 1);

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
                    .insert(Tile::new(matrix[i][j].num, (matrix[i][j].pos.1, matrix[i][j].pos.0)));
                }
            }
        }
    }

    // Move tiles left if A was pressed
    if keyboard_input.just_released(KeyCode::A) {
        // Removes all entitys
        query.for_each(|(entity, _, _)| {
            commands.entity(entity).despawn_recursive();
        });

        let mut matrix: [[Tile; BOARD_SIZE]; BOARD_SIZE] = get_matrix(&tiles);

        merge(&mut matrix, 0);

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
                    .insert(Tile::new(matrix[i][j].num, (matrix[i][j].pos.1, matrix[i][j].pos.0)));
                }
            }
        }
    }  

    if tiles.len() == 0 {
        let mut tile_position = Vec2::new(
            super::OFFSET + 0 as f32 * (super::SQUARE_SIZE),
            super::OFFSET + 0 as f32 * (super::SQUARE_SIZE),
        );    

        commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("4.png"),
            transform: Transform {
                translation: tile_position.extend(1.0),
                scale: Vec3::new(0.3, 0.3, 1.0),
                ..default()
            },
            ..default()
        })
        .insert(Tile::new(4, (0, 0)));

        tile_position = Vec2::new(
            super::OFFSET + 3 as f32 * (super::SQUARE_SIZE),
            super::OFFSET + 3 as f32 * (super::SQUARE_SIZE),
        );    

        commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("4.png"),
            transform: Transform {
                translation: tile_position.extend(1.0),
                scale: Vec3::new(0.3, 0.3, 1.0),
                ..default()
            },
            ..default()
        })
        .insert(Tile::new(4, (3, 3)));
    }

    if keyboard_input.just_released(KeyCode::I) {
        let mut arr = get_matrix(&tiles);
        set_position(&mut arr);

        for i in 0..arr.len() {
            for j in 0..arr[i].len() {
                print!("{:?}", arr[i][j]);
            }
            println!()
        }

        println!("-------------------------------------------------");
    }
}

fn get_matrix(tiles: &Vec<Tile>) -> [[Tile; BOARD_SIZE]; BOARD_SIZE] {
    let mut sorted: [[Tile; BOARD_SIZE]; BOARD_SIZE] = [[Tile::new(0, (0, 0)); BOARD_SIZE]; BOARD_SIZE];

    for tile in tiles {
        sorted[tile.pos.1 as usize][tile.pos.0 as usize] = *tile;
    }

    sorted
}

fn merge(tiles: &mut [[Tile; BOARD_SIZE]; BOARD_SIZE], direction: u8) {
    let mut idx: usize = 0;
    
    match direction {
        0 => { // bottom to top
            // Push together

            for i in 0..tiles.len() {
                for j in 0..tiles[i].len() {
                    print!("{:?}", tiles[i][j]);
                }
                println!()
            }
    
            println!("-------------------------------------------------");

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

            for i in 0..tiles.len() {
                for j in 0..tiles[i].len() {
                    print!("{:?}", tiles[i][j]);
                }
                println!()
            }
    
            println!("-------------------------------------------------");


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

            for i in 0..tiles.len() {
                for j in 0..tiles[i].len() {
                    print!("{:?}", tiles[i][j]);
                }
                println!()
            }
    
            println!("-------------------------------------------------");

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

            for i in 0..tiles.len() {
                for j in 0..tiles[i].len() {
                    print!("{:?}", tiles[i][j]);
                }
                println!()
            }
    
            println!("-------------------------------------------------");
        },
        1 => { // top to bottom
            // Push together

            for i in 1..tiles.len() {
                for j in 0..tiles[i].len() {
                    if tiles[tiles.len() - i - 1][j].num != 0 {
                        while tiles[(tiles.len() - i - 1) + (idx + 1)][j].num == 0 {
                            if idx < i - 1 {
                                idx += 1;
                            } else {
                                idx += 1;
                                break;
                            }
                        }
                    }

                    if tiles.len() - i != tiles.len() - i + idx {
                        tiles[tiles.len() - i - 1 + idx][j].num = tiles[tiles.len() - i - 1][j].num;
                        tiles[tiles.len() - i - 1][j].num = 0;
                    }

                    idx = 0;
                }
            }

            // Merge

            for i in 1..tiles.len() {
                for j in 0..tiles.len() {
                    if tiles[tiles.len() - i][j].num == tiles[tiles.len() - i - 1][j].num {
                        tiles[tiles.len() - i][j].num *= 2;
                        tiles[tiles.len() - i - 1][j].num = 0;
                    } 
                }
            }

            // Push together again

            for i in 1..tiles.len() {
                for j in 0..tiles[i].len() {
                    if tiles[tiles.len() - i - 1][j].num != 0 {
                        while tiles[(tiles.len() - i - 1) + (idx + 1)][j].num == 0 {
                            if idx < i - 1 {
                                idx += 1;
                            } else {
                                idx += 1;
                                break;
                            }
                        }
                    }

                    if tiles.len() - i != tiles.len() - i + idx {
                        tiles[tiles.len() - i - 1 + idx][j].num = tiles[tiles.len() - i - 1][j].num;
                        tiles[tiles.len() - i - 1][j].num = 0;
                    }

                    idx = 0;
                }
            }

            set_position(tiles);
        },
        2 => { // right to left
            // Push together
            for i in 0..tiles.len() {
                for j in 0..tiles[i].len() - 1 {
                    if tiles[i][j].num != 0 {
                        while tiles[i][j + (idx + 1)].num == 0 {
                            if idx + 1 < tiles[i].len() - j - 1 {
                                idx += 1;
                            } else {
                                idx += 1;
                                break;
                            }
                        }

                        if j != j + idx {
                            tiles[i][j + idx].num = tiles[i][j].num;
                            tiles[i][j].num = 0;
                        }

                        idx = 0;
                    }
                }
            }

            // Merge

            for i in 0..tiles.len() {
                for j in 0..tiles[i].len()  - 1 {
                    if tiles[i][j].num == tiles[i][j + 1].num {
                        tiles[i][j + 1].num *= 2;
                        tiles[i][j].num = 0;
                    }
                }
            }

            // Push together again

            for i in 0..tiles.len() {
                for j in 0..tiles[i].len() - 1 {
                    if tiles[i][j].num != 0 {
                        while tiles[i][j + (idx + 1)].num == 0 {
                            if idx + 1 < tiles[i].len() - j - 1 {
                                idx += 1;
                            } else {
                                idx += 1;
                                break;
                            }
                        }

                        if j != j + idx {
                            tiles[i][j + idx].num = tiles[i][j].num;
                            tiles[i][j].num = 0;
                        }

                        idx = 0;
                    }
                }
            }

            set_position(tiles);
        },
        _ => {
            
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