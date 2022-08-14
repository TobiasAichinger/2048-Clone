use bevy::prelude::*;

use crate::states::GameState;

const TEXT_COLOR: Color = Color::BLACK;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<Score>()
            .add_system_set(
                SystemSet::on_enter(GameState::Playing)
                    .with_system(spawn_score)
            )
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(update_score)
            )
            .add_system_set(
                SystemSet::on_exit(GameState::Playing)
                    .with_system(cleanup_score)
            );
    }
}

pub struct ScorePlugin;

#[derive(Default, Component, Debug, Clone, Copy)]
pub struct Score(pub u16);

impl std::fmt::Display for Score {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

fn spawn_score(
    mut commands: Commands,
    score: Res<Score>,
    materials: ResMut<AssetServer>
) {
    let text_style = TextStyle {
        font: materials.load("fonts/FiraSans-Bold.ttf"),
        font_size: 60.0,
        color: TEXT_COLOR,
    };

    let box_size = Vec2::new(300.0, 200.0);
    let box_position = Vec2::new(-(super::BOARD as f32 * super::SQUARE_SIZE + 30.0 + super::OFFSET), super::BOARD as f32 * super::SQUARE_SIZE + 50.0 + super::OFFSET);

    commands.spawn_bundle(Text2dBundle {
        text: Text::from_section(score.to_string(), text_style),
        text_2d_bounds: bevy::text::Text2dBounds {
            size: box_size,
        },
        transform: Transform::from_xyz(
            box_position.x,
            box_position.y,
            1.0,
        ),
        ..default()
    })
    .insert(Score(0));
}

fn update_score(
    mut score_query: Query<(&mut Text, &Score)>
) {
    for (mut text, score) in score_query.iter_mut() {
        info!("{}", score.to_string());
        text.sections[0].value = score.to_string();
    }
}

fn cleanup_score(
    mut commands: Commands,
    score: Query<(Entity, &Score)>
) {
    for (entity, _) in score.iter() {
        commands.entity(entity).despawn_recursive();
    }
}