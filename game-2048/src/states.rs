use bevy::prelude::*;

use crate::tile::TilePlugin;
use crate::menu::MenuPlugin;
use crate::score::ScorePlugin;
use crate::ai::AiPlugin;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    Menu,
    Playing,
    Ai,
}

#[derive(Component)]
pub struct Game;

impl Plugin for Game {
    fn build(&self, app: &mut App) {
        app
        .add_state(GameState::Menu)
        .add_plugins(DefaultPlugins)
        .add_plugin(MenuPlugin)
        .add_plugin(ScorePlugin)
        .add_plugin(TilePlugin)  
        .add_plugin(AiPlugin);  
    }
}