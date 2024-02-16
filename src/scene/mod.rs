pub(crate) mod pipes;

use bevy::pbr::{DirectionalLightShadowMap, NotShadowCaster};
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use pipes::PipePair;

use bevy_turborand::prelude::*;

pub struct ScenePlugin;

impl Plugin for ScenePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<PipePair>()
            .register_type::<SceneSettings>()
            .insert_resource(SceneSettings {
                pipe_gap_x: 7.0,
                pipe_gap_y: 3.0,
                pipe_spread: 4.0,
                pipe_speed: 0.0,
            })
            .insert_resource(AmbientLight {
                color: Color::WHITE,
                brightness: 0.25,
            })
            .insert_resource(DirectionalLightShadowMap { size: 4096 })
            .insert_resource(GlobalRng::new())
            .add_state::<AssetState>()
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
    // todo: Disable physics while assets are loading

    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(-2.5, 4.5, 9.0)
                .looking_at(Vec3::new(0.0, 1.0, 0.0), Vec3::Y),
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
            .looking_at(Vec3::new(-0.25, 0.0, -0.05), Vec3::Z),
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
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

pub fn spawn_level(mut commands: Commands, scene_settings: Res<SceneSettings>) {
    for i in 0..5 {
        commands.add(pipes::SpawnPipePair {
            position_x: (i) as f32 * scene_settings.pipe_gap_x,
        });
    }
}

fn move_pipes(
    mut pipe_query: Query<&mut Transform, With<PipePair>>,
    time: Res<Time>,
    scene_settings: Res<SceneSettings>,
) {
    for mut pipe_set in pipe_query.iter_mut() {
        pipe_set.translation.x -= scene_settings.pipe_speed * time.delta_seconds();
    }
}

// Respawn the pipes if they have gone off the screen past the player
fn recycle_pipes(
    mut pipe_query: Query<&mut Transform, With<PipePair>>,
    scene_settings: Res<SceneSettings>,
    mut rng_resource: ResMut<GlobalRng>,
) {
    let num_pipes = pipe_query.iter().len() as f32;
    let pipe_gap_x = scene_settings.pipe_gap_x;
    let out_of_view_bound = -2.0 * pipe_gap_x;

    for mut pipe_set in pipe_query.iter_mut() {
        if pipe_set.translation.x < out_of_view_bound {
            let random_num = rng_resource.f32();

            pipe_set.translation.x = pipe_gap_x * (num_pipes - 2.0);
            pipe_set.translation.y = (random_num * scene_settings.pipe_spread) - 2.0;
        }
    }
}
