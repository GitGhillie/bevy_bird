mod camera;
mod controls;
pub(crate) mod inputs;

use crate::scene::SceneSettings;
use bevy::prelude::*;
use bevy::transform::TransformSystem::TransformPropagate;
use bevy_xpbd_3d::components::{Collider, RigidBody};
use bevy_xpbd_3d::prelude::*;
use bevy_xpbd_3d::PhysicsSet;
use leafwing_input_manager::prelude::*;

#[derive(Reflect, Resource, Default)]
#[reflect(Resource)]
pub struct PlayerSettings {
    jump_velocity: f32,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<inputs::Action>::default())
            .register_type::<PlayerSettings>()
            .insert_resource(PlayerSettings {
                jump_velocity: 10.0,
            })
            .add_state::<GameState>()
            .add_systems(Startup, setup)
            .add_systems(OnEnter(GameState::Playing), start_game)
            .add_systems(OnEnter(GameState::Dead), end_game)
            .add_systems(
                PostUpdate,
                controls::check_for_game_start.run_if(in_state(GameState::Ready)),
            )
            .add_systems(Update, (controls::jump, print_collisions)) //todo: Sometimes starting the game with jump will not actually jump the player
            .add_systems(
                PostUpdate,
                camera::follow_player
                    .after(PhysicsSet::Sync)
                    .before(TransformPropagate),
            );
    }
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum GameState {
    #[default]
    Ready,
    Playing,
    Dead,
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Player;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Name::new("Player"),
        Player,
        RigidBody::Dynamic,
        GravityScale(4.0),
        LockedAxes::new()
            .lock_translation_x()
            .lock_translation_z()
            .lock_translation_y(),
        LinearVelocity::ZERO,
        Collider::ball(0.5),
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::UVSphere {
                radius: 0.5,
                sectors: 16,
                stacks: 8,
            })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.9).into()),
            transform: Transform::from_xyz(0.0, 2.0, 0.0),
            ..default()
        },
        InputManagerBundle::<inputs::Action> {
            input_map: inputs::create_input_map(),
            ..default()
        },
    ));
}

fn print_collisions(
    collision_event_reader: EventReader<Collision>,
    mut scene_settings: ResMut<SceneSettings>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if !collision_event_reader.is_empty() {
        scene_settings.pipe_speed = 0.0;
        next_state.set(GameState::Dead);
    }
}

fn start_game(
    mut commands: Commands,
    mut player_query: Query<Entity, With<LockedAxes>>,
    mut scene_settings: ResMut<SceneSettings>,
) {
    for player in &mut player_query {
        println!("Starting Game");
        commands
            .entity(player)
            .insert(LockedAxes::new().lock_translation_x().lock_translation_z());
        scene_settings.pipe_speed = 10.0;
    }
}

fn end_game(mut player_query: Query<&mut LockedAxes>, mut scene_settings: ResMut<SceneSettings>) {
    for player in &mut player_query {
        println!("Ending game");
        player.lock_translation_y();
        scene_settings.pipe_speed = 0.0;
    }
}
