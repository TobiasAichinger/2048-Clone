use bevy::prelude::*;

use crate::tile::Tile;
use crate::states::GameState;

pub struct AiPlugin;

impl Plugin for AiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                SystemSet::on_enter(GameState::Ai)
                    .with_system(setup_ai)
            )
            .add_system_set(
                SystemSet::on_update(GameState::Ai)
                    .with_system(solve)
            );
    }
}

fn setup_ai() {

}

fn solve(
    mut board: Query<(Entity, &mut Transform, &mut Tile)>,
) {
    
}