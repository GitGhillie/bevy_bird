use crate::gameplay::{GameState, ScoreInfo, ScoredEvent};
use bevy::prelude::*;

use bevy::color::palettes::css::GOLD;

pub struct ScoreTextPlugin;

impl Plugin for ScoreTextPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, update_score)
            .add_systems(OnEnter(GameState::Ready), update_high_score);
    }
}

#[derive(Component)]
struct ScoreText;

#[derive(Component)]
struct HighScoreText;

fn setup(mut commands: Commands) {
    commands.spawn((
        TextBundle::from_section(
            "0",
            TextStyle {
                font_size: 100.0,
                color: GOLD.into(),
                ..default()
            },
        )
        .with_text_justify(JustifyText::Center)
        .with_style(Style {
            position_type: PositionType::Absolute,
            justify_self: JustifySelf::Center,
            ..default()
        }),
        ScoreText,
    ));

    // todo: Also need to disable file creation etc
    //if !cfg!(target_arch = "wasm32") {
    commands.spawn((
        TextBundle::from_section(
            "0",
            TextStyle {
                font_size: 50.0,
                color: Color::WHITE,
                ..default()
            },
        )
        .with_text_justify(JustifyText::Right)
        .with_style(Style {
            position_type: PositionType::Absolute,
            justify_self: JustifySelf::End,
            ..default()
        }),
        HighScoreText,
    ));
}

fn update_score(
    mut score_query: Query<&mut Text, With<ScoreText>>,
    mut scored_event: EventReader<ScoredEvent>,
    score_info: Res<ScoreInfo>,
) {
    for _ in scored_event.read() {
        for mut text in &mut score_query {
            text.sections[0].value = format!("{num}", num = score_info.current_score);
        }
    }
}

fn update_high_score(
    mut high_score_query: Query<&mut Text, With<HighScoreText>>,
    score_info: Res<ScoreInfo>,
) {
    for mut text in &mut high_score_query {
        text.sections[0].value = format!("{num} ", num = score_info.high_score);
    }
}
