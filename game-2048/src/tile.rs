use bevy::prelude::*;

pub struct TilePlugin;

#[derive(Component, Debug, Clone)]
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
        query.for_each_mut( | (entity, mut transforming_tile, mut tile) | {
            position_changed = false;
            position_free = true;
            info!("Starting tile: {:?}", tile);

            while position_free {
                for idx in 0..tiles.len() {
                    if tiles[idx].pos == tile.pos {
                        continue;
                    }

                    if tile.pos.1 + 1 == tiles[idx].pos.1 && tile.pos.0 == tiles[idx].pos.0 && tile.num == tiles[idx].num {
                        commands.entity(entity).despawn_recursive();
                    }
    
                    if tile.pos.1 + 1 == tiles[idx].pos.1 && tile.pos.0 == tiles[idx].pos.0
                    || tile.pos.1 + 1 > 3 { // Check if one position above is free
                        position_free = false;
                    }
                }
    
                if position_free {
                    tile.pos.1 += 1;
                    position_changed = true;
                }
            }
    
            if position_changed {
                transform_x += tile.pos.0 as f32 * super::SQUARE_SIZE;
                transform_y += tile.pos.1 as f32 * super::SQUARE_SIZE;
        
                transforming_tile.translation = Vec3::new(transform_x, transform_y, 1.0);
            }

            info!("Out tile: {:?}", tile);
            info!("-------------------");
        });
    }   
    
    // Move tiles down if S was pressed
    if keyboard_input.just_released(KeyCode::S) {
        query.for_each_mut( | (entity, mut transforming_tile, mut tile) | {
            position_changed = false;
            position_free = true;
            info!("Starting tile: {:?}", tile);

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

            info!("Out tile: {:?}", tile);
            info!("-------------------");
        });
    } 

    // Move tiles right if D was pressed
    if keyboard_input.just_released(KeyCode::D) {
        query.for_each_mut( | (entity, mut transforming_tile, mut tile) | {
            position_changed = false;
            position_free = true;
            info!("Starting tile: {:?}", tile);

            while position_free {
                for idx in 0..tiles.len() {
                    if tiles[idx].pos == tile.pos {
                        continue;
                    }
    

                    if tile.pos.1 + 1 == tiles[idx].pos.1 && tile.pos.0 == tiles[idx].pos.0 && tile.num == tiles[idx].num {
                        commands.entity(entity).despawn_recursive();
                    } else if tile.pos.0 + 1 == tiles[idx].pos.0 && tile.pos.1 == tiles[idx].pos.1
                    || tile.pos.0 + 1 > 3 { // Check if one position above is free
                        position_free = false;
                    }
                }
    
                if position_free {
                    tile.pos.0 += 1;
                    position_changed = true;
                }
            }
    
            if position_changed {
                transform_x += tile.pos.0 as f32 * super::SQUARE_SIZE;
                transform_y += tile.pos.1 as f32 * super::SQUARE_SIZE;
        
                transforming_tile.translation = Vec3::new(transform_x, transform_y, 1.0);
            }

            info!("Out tile: {:?}", tile);
            info!("-------------------");
        });
    }   

    // Move tiles left if A was pressed
    if keyboard_input.just_released(KeyCode::A) {
        query.for_each_mut( | (entity, mut transforming_tile, mut tile) | {
            position_changed = false;
            position_free = true;
            info!("Starting tile: {:?}", tile);

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

            info!("Out tile: {:?}", tile);
            info!("-------------------");
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
            texture: asset_server.load("two.png"),
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
        info!("{:?}", tiles);
    }   
}