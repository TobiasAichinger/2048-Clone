use bevy::prelude::*;

use crate::states::GameState;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                SystemSet::on_enter(GameState::Playing)
                    .with_system(spawn_score)
            );
    }
}

pub struct ScorePlugin;

#[derive(Default, Component, Debug, Clone, Copy)]
pub struct Score(u16);

fn spawn_score(
    mut commands: Commands,
) {
    
}

fn update_score(
    mut score: Query<&mut Score>,
    mut commands: Commands,
) {

}