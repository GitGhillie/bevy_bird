use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

use crate::gameplay::{JumpedEvent, ScoredEvent};

pub struct GameAudioPlugin;

impl Plugin for GameAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(AudioPlugin)
            .add_systems(Update, (score_audio, jump_audio));
    }
}

fn score_audio(
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    mut score_event: EventReader<ScoredEvent>,
) {
    for _ in score_event.read() {
        audio.play(asset_server.load("audio/pickupCoin.ogg"));
    }
}

fn jump_audio(
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    mut jump_event: EventReader<JumpedEvent>,
) {
    for _ in jump_event.read() {
        audio.play(asset_server.load("audio/explosion.ogg"));
    }
}
