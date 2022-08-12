use bevy::prelude::*;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(spawn_camera);
    }
}

pub struct MenuPlugin;

fn spawn_camera(
    mut commands: Commands,
) {
    commands.spawn_bundle(Camera2dBundle::default());
}