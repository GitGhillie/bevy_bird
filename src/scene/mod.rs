mod pipes;

use bevy::pbr::DirectionalLightShadowMap;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use pipes::PipesMarker;

pub struct ScenePlugin;

impl Plugin for ScenePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(DirectionalLightShadowMap { size: 4096 })
            .add_state::<GameState>()
            .add_loading_state(
                LoadingState::new(GameState::AssetLoading)
                    .continue_to_state(GameState::AssetsLoaded)
                    .load_collection::<SceneAssets>(),
            )
            .register_type::<PipesMarker>()
            .add_systems(Startup, setup)
            .add_systems(OnEnter(GameState::AssetsLoaded), spawn_level)
            .add_systems(Update, move_pipes);
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
enum GameState {
    #[default]
    AssetLoading,
    AssetsLoaded,
}

#[derive(AssetCollection, Resource)]
struct SceneAssets {
    #[asset(path = "objects/pipe.glb#Scene0")]
    pipe: Handle<Scene>,
}

fn setup(mut commands: Commands) {
    // todo: Disable physics while assets are loading

    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_xyz(0.0, 1.0, 0.0)
            .looking_at(Vec3::new(-0.8, 0.0, -0.4), Vec3::Z),
        ..default()
    });
}

fn spawn_level(mut commands: Commands) {
    let pipe_gap_x = 7.0;

    for i in 0..5 {
        commands.add(pipes::SpawnPipe {
            position_x: i as f32 * pipe_gap_x,
        });
    }
}

fn move_pipes(mut pipe_query: Query<&mut Transform, With<PipesMarker>>, time: Res<Time>) {
    for mut pipe_set in pipe_query.iter_mut() {
        pipe_set.translation.x -= 10.0 * time.delta_seconds();
    }
}
