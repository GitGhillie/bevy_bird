use bevy::pbr::DirectionalLightShadowMap;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
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
            .register_type::<MarkerComponent>()
            .add_systems(Startup, setup)
            .add_systems(OnEnter(GameState::AssetsLoaded), spawn_level);
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
    // #[asset(path = "levels/level1/detail.glb#Scene0")]
    // detail: Handle<Scene>,
}

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
struct MarkerComponent;

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

fn spawn_level(mut commands: Commands, scene_assets: Res<SceneAssets>) {
    let parent = commands
        .spawn((
            Name::from("Pipe"),
            SceneBundle {
                scene: scene_assets.pipe.clone_weak(),
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                ..default()
            },
            RigidBody::Static,
        ))
        .id();

    let collider_length = 10.0;
    let child = commands
        .spawn((
            Collider::cuboid(2.0, collider_length, 2.0),
            Transform::from_xyz(0.0, -collider_length / 2.0, 0.0),
        ))
        .id();

    commands.entity(parent).add_child(child);
}
