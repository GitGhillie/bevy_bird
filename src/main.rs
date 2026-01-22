#![allow(clippy::too_many_arguments, clippy::type_complexity)]

mod audio;
mod gameplay;
mod player;
mod scene;
mod score_save;
mod ui;

use avian3d::prelude::*;
use bevy::asset::AssetMetaCheck;
use bevy::prelude::*;

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

    // todo mipmaps
    #[cfg(feature = "desktop")]
    {
        app.add_plugins(score_save::SavePlugin);
    }

    #[cfg(feature = "debugging")]
    {
        use bevy_inspector_egui::bevy_egui::EguiPlugin;
        use bevy_inspector_egui::quick::WorldInspectorPlugin;

        app.add_plugins(PhysicsDebugPlugin::default())
            .add_plugins(EguiPlugin::default())
            .add_plugins(WorldInspectorPlugin::default());

        //bevy_mod_debugdump::print_schedule_graph(&mut app, Update);
    }

    app.run();
}
