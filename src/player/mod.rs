mod camera;
mod controls;
pub(crate) mod inputs;

use crate::scene::{spawn_level, SceneSettings};
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
            .add_systems(OnEnter(GameState::Ready), spawn_level)
            .add_systems(OnEnter(GameState::Playing), start_game)
            .add_systems(OnEnter(GameState::Dead), end_game)
            .add_systems(
                PostUpdate,
                controls::check_for_game_start.run_if(in_state(GameState::Ready)),
            )
            .add_systems(Update, controls::jump) //todo: Sometimes starting the game with jump will not actually jump the player
            .add_systems(
                Update,
                check_for_collisions.run_if(in_state(GameState::Playing)),
            )
            .add_systems(
                PostUpdate,
                camera::follow_player
                    .after(PhysicsSet::Sync)
                    .before(TransformPropagate),
            );
    }
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
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

fn check_for_collisions(
    mut collision_event_reader: EventReader<Collision>,
    mut scene_settings: ResMut<SceneSettings>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if !collision_event_reader.is_empty() {
        // Drain events so they don't cause issues later
        for _ in collision_event_reader.read() {}
        println!("Ending game after collision");
        scene_settings.pipe_speed = 0.0;
        next_state.set(GameState::Dead);
    }
}

fn start_game(
    mut commands: Commands,
    mut player_query: Query<Entity, With<LockedAxes>>,
    mut scene_settings: ResMut<SceneSettings>,
) {
    println!("Starting Game");
    scene_settings.pipe_speed = 10.0;

    for player in &mut player_query {
        commands
            .entity(player)
            .insert(LockedAxes::new().lock_translation_x().lock_translation_z());
    }
}

fn end_game(
    mut player_query: Query<Entity, With<LockedAxes>>,
    mut scene_settings: ResMut<SceneSettings>,
    mut next_state: ResMut<NextState<GameState>>,
    mut commands: Commands,
    pipe_query: Query<Entity, With<crate::scene::pipes::PipesMarker>>,
) {
    println!("Ending game");
    scene_settings.pipe_speed = 0.0;

    for player in &mut player_query {
        commands.entity(player).insert(
            LockedAxes::new()
                .lock_translation_x()
                .lock_translation_z()
                .lock_translation_y(),
        );
    }

    next_state.set(GameState::Ready);
    //todo move this beun somewhere else
    scene_settings.pipe_speed = 0.0;
    for ent in pipe_query.iter() {
        commands.entity(ent).despawn_recursive();
    }
}
