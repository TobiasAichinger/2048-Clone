use bevy::prelude::*;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<Score>();
    }
}

pub struct ScorePlugin;

#[derive(Default, Debug, Clone, Copy)]
pub struct Score(u16);

