#![allow(clippy::too_many_arguments, clippy::type_complexity)]

mod audio;
mod gameplay;
mod player;
mod scene;
mod ui;

use bevy::prelude::*;
use bevy_xpbd_3d::prelude::*;

#[cfg(feature = "debugging")]
use {
    bevy_inspector_egui::quick::WorldInspectorPlugin,
    bevy_screen_diagnostics::{ScreenDiagnosticsPlugin, ScreenFrameDiagnosticsPlugin},
};

fn main() {
    let mut app = App::new();

    app
        //.insert_resource(AssetMetaCheck::Never) // I think this is needed for a web release
        .add_plugins(DefaultPlugins)
        .add_plugins(PhysicsPlugins::default())
        .add_plugins(player::PlayerPlugin)
        .add_plugins(scene::ScenePlugin)
        .add_plugins(gameplay::StateTransitionPlugin)
        .add_plugins(ui::ScoreTextPlugin)
        .add_plugins(audio::GameAudioPlugin);

    #[cfg(feature = "debugging")]
    {
        app.add_plugins(ScreenDiagnosticsPlugin::default())
            .add_plugins(ScreenFrameDiagnosticsPlugin)
            .add_plugins(PhysicsDebugPlugin::default())
            .add_plugins(WorldInspectorPlugin::default());
    }

    app.run();
}
