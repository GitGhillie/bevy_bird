use bevy::ecs::system::Command;
use bevy::pbr::DirectionalLightShadowMap;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_xpbd_3d::math::PI;
use bevy_xpbd_3d::prelude::*;

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

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
struct PipesMarker;

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

struct SpawnPipe {
    pub position_x: f32,
}

impl Command for SpawnPipe {
    fn apply(self, world: &mut World) {
        let assets = world.get_resource::<SceneAssets>();

        if let Some(assets) = assets {
            let collider_length = 10.0;
            let pipe_gap_y = 3.0;

            let pipe_handle = assets.pipe.clone_weak();

            let transform_lower = Transform::from_xyz(0.0, 0.0, 0.0);
            let mut transform_upper = Transform::from_xyz(0.0, pipe_gap_y, 0.0);
            transform_upper.rotate_local_z(PI);

            let parent_components = (
                Name::from("PipeParent"),
                PipesMarker,
                VisibilityBundle::default(),
                TransformBundle {
                    local: Transform::from_xyz(self.position_x, 0.0, 0.0),
                    ..default()
                },
            );

            world.spawn(parent_components).with_children(|parent| {
                let transforms = [transform_lower, transform_upper];
                for transform in transforms {
                    let pipe_components = (
                        Name::from("Pipe"),
                        RigidBody::Kinematic,
                        SceneBundle {
                            scene: pipe_handle.clone_weak(),
                            transform,
                            ..default()
                        },
                    );

                    let collider_components = (
                        Collider::cuboid(2.0, collider_length, 2.0),
                        Transform::from_xyz(0.0, -collider_length / 2.0, 0.0),
                    );

                    parent.spawn(pipe_components).with_children(|parent| {
                        parent.spawn(collider_components);
                    });
                }
            });
        }
    }
}

fn spawn_level(mut commands: Commands) {
    let pipe_gap_x = 7.0;

    for i in 0..5 {
        commands.add(SpawnPipe {
            position_x: i as f32 * pipe_gap_x,
        });
    }
}

fn move_pipes(mut pipe_query: Query<&mut Transform, With<PipesMarker>>, time: Res<Time>) {
    for mut pipe_set in pipe_query.iter_mut() {
        pipe_set.translation.x -= 10.0 * time.delta_seconds();
    }
}
