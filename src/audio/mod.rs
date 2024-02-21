use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_kira_audio::prelude::*;

use crate::gameplay::{JumpedEvent, ScoredEvent};

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
enum AssetState {
    #[default]
    Loading,
    Loaded,
}

#[derive(AssetCollection, Resource)]
struct AudioAssets {
    #[asset(path = "audio/pickupCoin.ogg")]
    coin: Handle<AudioSource>,
    #[asset(path = "audio/explosion.ogg")]
    gun: Handle<AudioSource>,
}

pub struct GameAudioPlugin;

impl Plugin for GameAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(AudioPlugin)
            .init_state::<AssetState>()
            .add_loading_state(
                LoadingState::new(AssetState::Loading)
                    .continue_to_state(AssetState::Loaded)
                    .load_collection::<AudioAssets>(),
            )
            .add_systems(
                Update,
                (score_audio, jump_audio).run_if(in_state(AssetState::Loaded)),
            );
    }
}

fn score_audio(
    audio_assets: Res<AudioAssets>,
    audio: Res<Audio>,
    mut score_event: EventReader<ScoredEvent>,
) {
    for _ in score_event.read() {
        audio.play(audio_assets.coin.clone_weak());
    }
}

fn jump_audio(
    audio_assets: Res<AudioAssets>,
    audio: Res<Audio>,
    mut jump_event: EventReader<JumpedEvent>,
) {
    for _ in jump_event.read() {
        audio.play(audio_assets.gun.clone_weak());
    }
}
