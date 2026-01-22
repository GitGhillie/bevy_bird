pub(crate) mod pipes;

use avian3d::math::PI;
use bevy::light::{CascadeShadowConfigBuilder, DirectionalLightShadowMap, NotShadowCaster};
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_rand::prelude::*;
use rand_core::RngCore;

use pipes::PipePair;

pub struct ScenePlugin;

impl Plugin for ScenePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<PipePair>()
            .register_type::<SceneSettings>()
            .insert_resource(SceneSettings {
                pipe_gap_x: 7.0,
                pipe_gap_y: 3.1,
                pipe_spread: 4.0,
                pipe_speed: 0.0,
            })
            .insert_resource(GlobalAmbientLight {
                color: Color::WHITE,
                brightness: 500.0,
                ..default()
            })
            .insert_resource(DirectionalLightShadowMap { size: 4096 })
            .add_plugins(EntropyPlugin::<WyRand>::default())
            .init_state::<AssetState>()
            .add_loading_state(
                LoadingState::new(AssetState::Loading)
                    .continue_to_state(AssetState::Loaded)
                    .load_collection::<SceneAssets>(),
            )
            .add_systems(Startup, setup)
            .add_systems(OnEnter(AssetState::Loaded), spawn_level)
            .add_systems(Update, (recycle_pipes, move_pipes));
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
enum AssetState {
    #[default]
    Loading,
    Loaded,
}

#[derive(AssetCollection, Resource)]
struct SceneAssets {
    #[asset(path = "objects/pipe.glb#Scene0")]
    pipe: Handle<Scene>,
}

#[derive(Reflect, Resource, Default)]
#[reflect(Resource)]
pub struct SceneSettings {
    pipe_gap_x: f32,
    pipe_gap_y: f32,
    pipe_spread: f32,
    pub pipe_speed: f32,
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::new(0.0, 1.0, 0.0), Vec3::Y),
        DistanceFog {
            color: Color::srgba(0.35, 0.48, 0.66, 1.0),
            directional_light_color: Color::srgba(1.0, 0.95, 0.85, 0.5),
            directional_light_exponent: 30.0,
            falloff: FogFalloff::from_visibility_colors(
                60.0, // distance in world units up to which objects retain visibility (>= 5% contrast)
                Color::srgb(0.35, 0.5, 0.66), // atmospheric extinction color (after light is lost due to absorption by atmospheric particles)
                Color::srgb(0.8, 0.844, 1.0), // atmospheric inscattering color (light gained due to scattering from the sun)
            ),
        },
    ));

    let shadow_config = CascadeShadowConfigBuilder {
        maximum_distance: 20.0,
        ..default()
    }
    .build();

    commands.spawn((
        DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(0.0, 1.0, 0.0).looking_at(Vec3::new(-0.25, 0.0, -0.05), Vec3::Z),
        shadow_config,
    ));

    commands.spawn((
        Mesh3d(meshes.add(Mesh::from(Cuboid::default()))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Srgba::hex("888888").unwrap().into(),
            unlit: true,
            cull_mode: None,
            ..default()
        })),
        Transform::from_scale(Vec3::splat(100.0)),
        NotShadowCaster,
    ));
}

pub fn spawn_level(mut commands: Commands, scene_settings: Res<SceneSettings>) {
    for i in 0..5 {
        commands.queue(pipes::SpawnPipePair {
            position_x: (i + 1) as f32 * scene_settings.pipe_gap_x,
            rotation: i as f32 * PI * 0.5,
        });
    }
}

fn move_pipes(
    mut pipe_query: Query<&mut Transform, With<PipePair>>,
    time: Res<Time>,
    scene_settings: Res<SceneSettings>,
) {
    for mut pipe_set in pipe_query.iter_mut() {
        pipe_set.translation.x -= scene_settings.pipe_speed * time.delta_secs();
    }
}

// Respawn the pipes if they have gone off the screen past the player
fn recycle_pipes(
    mut pipe_query: Query<&mut Transform, With<PipePair>>,
    scene_settings: Res<SceneSettings>,
    mut rng_resource: Single<&mut WyRand, With<GlobalRng>>,
) {
    let num_pipes = pipe_query.iter().len() as f32;
    let pipe_gap_x = scene_settings.pipe_gap_x;
    let out_of_view_bound = -2.0 * pipe_gap_x;

    for mut pipe_set in pipe_query.iter_mut() {
        if pipe_set.translation.x < out_of_view_bound {
            // Create random f32 between 0.0 and 1.0
            let random_num = if cfg!(feature = "max_difficulty") {
                (rng_resource.next_u32() as f32 / u32::MAX as f32).round()
            } else {
                rng_resource.next_u32() as f32 / u32::MAX as f32
            };

            pipe_set.translation.x = pipe_gap_x * (num_pipes - 2.0);
            pipe_set.translation.y = (random_num * scene_settings.pipe_spread) - 2.5;
        }
    }
}
