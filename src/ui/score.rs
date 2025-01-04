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
        Text::new("0"),
        TextFont::from_font_size(100.0),
        TextColor(GOLD.into()),
        TextLayout::new_with_justify(JustifyText::Center),
        Node {
            position_type: PositionType::Absolute,
            justify_self: JustifySelf::Center,
            ..default()
        },
        ScoreText,
    ));

    // todo: Also need to disable file creation etc
    //if !cfg!(target_arch = "wasm32") {
    commands.spawn((
        Text::new("0"),
        TextFont::from_font_size(75.0),
        TextColor::WHITE,
        TextLayout::new_with_justify(JustifyText::Right),
        Node {
            position_type: PositionType::Absolute,
            justify_self: JustifySelf::End,
            ..default()
        },
        HighScoreText,
    ));
}

fn update_score(
    score_query: Query<Entity, With<ScoreText>>,
    mut scored_event: EventReader<ScoredEvent>,
    score_info: Res<ScoreInfo>,
    mut writer: TextUiWriter,
) {
    for _ in scored_event.read() {
        for text_ent in &score_query {
            *writer.text(text_ent, 0) = format!("{num}", num = score_info.current_score);
        }
    }
}

fn update_high_score(
    high_score_query: Query<Entity, With<HighScoreText>>,
    score_info: Res<ScoreInfo>,
    mut writer: TextUiWriter,
) {
    for text_ent in &high_score_query {
        *writer.text(text_ent, 0) = format!("{num} ", num = score_info.high_score);
    }
}
