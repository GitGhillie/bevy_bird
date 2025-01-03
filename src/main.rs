#![allow(clippy::too_many_arguments, clippy::type_complexity)]

mod audio;
mod gameplay;
mod player;
mod scene;
mod score_save;
mod ui;

use bevy::asset::AssetMetaCheck;
use bevy::prelude::*;
use bevy_xpbd_3d::prelude::*;

#[cfg(feature = "debugging")]
use {
    bevy_inspector_egui::quick::WorldInspectorPlugin,
    bevy_screen_diagnostics::{ScreenDiagnosticsPlugin, ScreenFrameDiagnosticsPlugin},
};

fn main() {
    let mut app = App::new();

    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    canvas: Some("#game-canvas".into()),
                    ..default()
                }),
                ..default()
            })
            .set(AssetPlugin {
                meta_check: AssetMetaCheck::Never, // Needed to prevent errors in web release
                ..default()
            }),
    )
    .add_plugins(PhysicsPlugins::default())
    .add_plugins(player::PlayerPlugin)
    .add_plugins(scene::ScenePlugin)
    .add_plugins(gameplay::StateTransitionPlugin)
    .add_plugins(ui::UiPlugin)
    .add_plugins(audio::GameAudioPlugin);

    // Since 0.12 mipmaps can be automatically generated but this did not work for me. TBD
    #[cfg(feature = "desktop")]
    {
        app.add_plugins(score_save::SavePlugin)
            .add_plugins(bevy_mod_mipmap_generator::MipmapGeneratorPlugin)
            .add_systems(
                Update,
                bevy_mod_mipmap_generator::generate_mipmaps::<StandardMaterial>,
            );
    }

    #[cfg(feature = "debugging")]
    {
        app.add_plugins(ScreenDiagnosticsPlugin::default())
            .add_plugins(ScreenFrameDiagnosticsPlugin)
            .add_plugins(PhysicsDebugPlugin::default())
            .add_plugins(WorldInspectorPlugin::default());

        //bevy_mod_debugdump::print_schedule_graph(&mut app, Update);
    }

    app.run();
}
