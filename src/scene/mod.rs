mod lights;

use bevy::pbr::DirectionalLightShadowMap;
use bevy::prelude::*;
use bevy::render::primitives::Aabb;
use bevy_asset_loader::prelude::*;
use bevy_scene_hook::{HookPlugin, HookedSceneBundle, SceneHook};
use bevy_xpbd_3d::prelude::*;

pub struct ScenePlugin;

impl Plugin for ScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(HookPlugin)
            .insert_resource(DirectionalLightShadowMap { size: 4096 })
            .add_state::<GameState>()
            .add_loading_state(
                LoadingState::new(GameState::AssetLoading)
                    .continue_to_state(GameState::AssetsLoaded)
                    .load_collection::<SceneAssets>(),
            )
            .register_type::<MarkerComponent>()
            .add_systems(Startup, setup)
            .add_systems(OnEnter(GameState::AssetsLoaded), spawn_blockout)
            .add_systems(Update, lights::replace_added_lights);
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
    #[asset(path = "levels/level1/blockout.glb#Scene0")]
    blockout: Handle<Scene>,
    #[asset(path = "levels/level1/detail.glb#Scene0")]
    detail: Handle<Scene>,
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
}

fn spawn_blockout(mut commands: Commands, scene_assets: Res<SceneAssets>) {
    commands.spawn((
        Name::from("Blockout"),
        SceneBundle {
            scene: scene_assets.blockout.clone_weak(),
            ..default()
        },
        AsyncSceneCollider::new(Some(ComputedCollider::ConvexHull)),
        RigidBody::Static,
    ));

    commands
        .spawn(HookedSceneBundle {
            scene: SceneBundle {
                scene: scene_assets.detail.clone_weak(),
                ..default()
            },
            hook: SceneHook::new(|entity, cmds| {
                if let Some(name) = entity.get::<Name>() {
                    // When importing from Blender each object will have a child with the actual mesh and Aabb.
                    // However, we only want to add our component to the parent object, so early return.
                    if entity.contains::<Aabb>() {
                        return;
                    }

                    if name.starts_with("Suzanne") {
                        cmds.insert(MarkerComponent);
                    }
                }
            }),
        })
        .insert(Name::from("Detail"));
}
