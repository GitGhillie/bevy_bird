mod pipes;

use bevy::pbr::{DirectionalLightShadowMap, NotShadowCaster};
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

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // todo: Disable physics while assets are loading

    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        FogSettings {
            color: Color::rgba(0.35, 0.48, 0.66, 1.0),
            directional_light_color: Color::rgba(1.0, 0.95, 0.85, 0.5),
            directional_light_exponent: 30.0,
            falloff: FogFalloff::from_visibility_colors(
                60.0, // distance in world units up to which objects retain visibility (>= 5% contrast)
                Color::rgb(0.35, 0.5, 0.66), // atmospheric extinction color (after light is lost due to absorption by atmospheric particles)
                Color::rgb(0.8, 0.844, 1.0), // atmospheric inscattering color (light gained due to scattering from the sun)
            ),
        },
    ));

    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_xyz(0.0, 1.0, 0.0)
            .looking_at(Vec3::new(-0.8, 0.0, -0.4), Vec3::Z),
        ..default()
    });

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::default())),
            material: materials.add(StandardMaterial {
                base_color: Color::hex("888888").unwrap(),
                unlit: true,
                cull_mode: None,
                ..default()
            }),
            transform: Transform::from_scale(Vec3::splat(100.0)),
            ..default()
        },
        NotShadowCaster,
    ));
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
