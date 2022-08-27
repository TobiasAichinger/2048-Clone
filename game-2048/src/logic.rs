use crate::tile::Tile;

pub struct Logic;

impl Logic {
    pub fn merge() {
        
    }

    pub fn get_matrix(tiles: &Vec<Tile>) -> [[Tile; super::BOARD]; super::BOARD] {
        let mut matrix: [[Tile; super::BOARD]; super::BOARD] = [[Tile::new(0, (3, 3)); super::BOARD]; super::BOARD];
    
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

    pub fn check_set_new_tile(matrix: [[Tile; super::BOARD]; super::BOARD], matrix_clone: [[Tile; super::BOARD]; super::BOARD]) -> bool {
        for i in 0..matrix.len() {
            for j in 0..matrix[i].len() {
                if matrix[i][j].num != matrix_clone[i][j].num {
                    return true; 
                }
            }
        }
    
        false
    }
}