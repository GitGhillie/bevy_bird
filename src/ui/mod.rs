use crate::gameplay::{ScoreInfo, ScoredEvent};
use bevy::prelude::*;

pub struct ScoreTextPlugin;

impl Plugin for ScoreTextPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, update_score);
    }
}

#[derive(Component)]
struct TextMarker;

fn setup(mut commands: Commands) {
    commands.spawn((
        TextBundle::from_section(
            "0",
            TextStyle {
                font_size: 100.0,
                color: Color::GOLD,
                ..default()
            },
        )
        .with_text_alignment(TextAlignment::Center)
        .with_style(Style {
            position_type: PositionType::Absolute,
            justify_self: JustifySelf::Center,
            ..default()
        }),
        TextMarker,
    ));
}

fn update_score(
    mut query: Query<&mut Text, With<TextMarker>>,
    mut scored_event: EventReader<ScoredEvent>,
    score_info: Res<ScoreInfo>,
) {
    for _ in scored_event.read() {
        for mut text in &mut query {
            text.sections[0].value = format!("{num}", num = score_info.current_score);
        }
    }
}
