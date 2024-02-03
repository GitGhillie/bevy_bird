// Bevy code commonly triggers these lints and they may be important signals
// about code quality. They are sometimes hard to avoid though, and the CI
// workflow treats them as errors, so this allows them throughout the project.
// Feel free to delete this line.
#![allow(clippy::too_many_arguments, clippy::type_complexity)]

mod player;
mod scene;

use bevy::log::LogPlugin;
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
        .add_plugins(DefaultPlugins.set(LogPlugin {
            filter:
                "info,wgpu_core=warn,wgpu_hal=warn,mygame=debug,bevy_gltf_components=debug".into(),
            level: bevy::log::Level::DEBUG,
        }))
        .add_plugins(PhysicsPlugins::default())
        .add_plugins(player::PlayerPlugin)
        .add_plugins(scene::ScenePlugin);

    #[cfg(feature = "debugging")]
    {
        app.add_plugins(ScreenDiagnosticsPlugin::default())
            .add_plugins(ScreenFrameDiagnosticsPlugin)
            .add_plugins(PhysicsDebugPlugin::default())
            .add_plugins(WorldInspectorPlugin::default());
    }

    app.run();
}
