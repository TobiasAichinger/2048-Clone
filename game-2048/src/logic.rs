use crate::tile::Tile;

pub struct Logic;

impl Logic {
    pub fn merge(matrix: &mut [[Tile; super::BOARD]; super::BOARD]) -> u16 {
        for i in 0..matrix.len() {
            for j in 0..matrix.len() {
                if matrix[i][j].num != 0 {
                    matrix[i][j].pos = (j + 1, i);
                }
            }
        }
        0
    }

    pub fn get_matrix(tiles: &Vec<Tile>) -> [[Tile; super::BOARD]; super::BOARD] {
        let mut matrix: [[Tile; super::BOARD]; super::BOARD] = [[Tile::new(0, 0, (3, 3)); super::BOARD]; super::BOARD];
    
        for tile in tiles {
            matrix[tile.pos.1 as usize][tile.pos.0 as usize] = *tile;
        }

        for i in 0..matrix.len() {
            for j in 0..matrix.len() {
                if matrix[i][j].num == 0 {
                    matrix[i][j].pos = (j, i);
                }
            }
        }
    
        matrix
    }

    pub fn check_set_new_tile(matrix: [[Tile; super::BOARD]; super::BOARD], matrix_clone: [[Tile; super::BOARD]; super::BOARD]) -> (bool, u8) {
        let mut new_tile: bool = false;
        let mut new_id: u8 = 0;

        for i in 0..matrix.len() {
            for j in 0..matrix[i].len() {
                if matrix[i][j].num != matrix_clone[i][j].num {
                    new_tile = true;
                }

                if matrix[i][j].id > new_id {
                    new_id = matrix[i][j].id + 1;
                }
            }
        }
    
        (new_tile, new_id)
    }
}