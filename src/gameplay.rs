use crate::player::PlayerSettings;
use crate::player::controls::{check_for_game_start, jump};
use crate::scene::pipes::PipePair;
use crate::scene::{SceneSettings, spawn_level};

use avian3d::math::Quaternion;
use avian3d::prelude::*;
use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    Ready,
    Playing,
    Dead,
}

#[derive(Resource, Default, Copy, Clone, Deserialize, Serialize)]
pub struct ScoreInfo {
    pub current_score: u32,
    pub high_score: u32,
}

// Indicates if a pipe has passed the player
#[derive(Component)]
struct Scored;

#[derive(Message)]
pub struct ScoredEvent;

#[derive(Message)]
pub struct JumpedEvent;

pub struct StateTransitionPlugin;

impl Plugin for StateTransitionPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .insert_resource(ScoreInfo::default())
            .add_message::<ScoredEvent>()
            .add_message::<JumpedEvent>()
            .add_systems(OnEnter(GameState::Ready), spawn_level)
            .add_systems(OnEnter(GameState::Playing), start_game)
            .add_systems(
                Update,
                (
                    end_game
                        .run_if(in_state(GameState::Dead).and(on_timer(Duration::from_secs(1))))
                        .after(check_for_collisions),
                    (check_for_game_start, force_no_rotation).run_if(in_state(GameState::Ready)),
                    jump.run_if(in_state(GameState::Playing)),
                    (ramp_up_speed, check_for_collisions, check_for_out_of_bounds)
                        .chain()
                        .run_if(in_state(GameState::Playing)),
                    scoring,
                ),
            );
    }
}

fn start_game(
    mut commands: Commands,
    mut player_query: Single<(Entity, &mut LinearVelocity), With<LockedAxes>>,
    mut scene_settings: ResMut<SceneSettings>,
    player_settings: Res<PlayerSettings>,
    mut score_info: ResMut<ScoreInfo>,
) {
    scene_settings.pipe_speed = 5.0;
    score_info.current_score = 0;

    let player = player_query.0;
    let velocity = &mut player_query.1;

    // Unlock the y translation
    commands.entity(player).insert(
        LockedAxes::ROTATION_LOCKED
            .lock_translation_x()
            .lock_translation_z(),
    );

    // We need to jump when starting the game since the jump action is 'used up' when
    // checking for the state transition from `Ready` to `Playing`.
    velocity.y = player_settings.jump_velocity;
}

fn ramp_up_speed(mut scene_settings: ResMut<SceneSettings>, time: Res<Time>) {
    let max_pipe_speed = 8.0;

    if scene_settings.pipe_speed < max_pipe_speed {
        scene_settings.pipe_speed += 0.2 * time.delta_secs();
    }

    #[cfg(feature = "max_difficulty")]
    {
        scene_settings.pipe_speed = max_pipe_speed;
    }
}

fn check_for_collisions(
    collisions: Collisions,
    mut scene_settings: ResMut<SceneSettings>,
    mut next_state: ResMut<NextState<GameState>>,
    player: Single<Entity, With<LockedAxes>>,
    mut commands: Commands,
) {
    if collisions.iter().next().is_some() {
        scene_settings.pipe_speed = 0.0;

        commands.entity(*player).insert((LockedAxes::new(),));

        next_state.set(GameState::Dead);
    }
}

fn check_for_out_of_bounds(
    mut next_state: ResMut<NextState<GameState>>,
    player: Single<&GlobalTransform, With<LockedAxes>>,
    mut scene_settings: ResMut<SceneSettings>,
) {
    if player.translation().y < -20.0 {
        scene_settings.pipe_speed = 0.0;
        next_state.set(GameState::Dead);
    }
}

// Quick hack to make sure the initial rotation is correct when starting the game
// Without it the physics engine will sometimes apply a rotation the first frame
// after respawning.
fn force_no_rotation(
    mut player_rotation: Single<&mut Rotation, With<LockedAxes>>,
    player_settings: Res<PlayerSettings>,
) {
    ***player_rotation = Quaternion::from_rotation_z(player_settings.initial_rotation);
}

fn end_game(
    mut commands: Commands,
    player: Single<Entity, With<LockedAxes>>,
    mut next_state: ResMut<NextState<GameState>>,
    pipe_query: Query<Entity, With<PipePair>>,
    player_settings: Res<PlayerSettings>,
) {
    // todo: Factor out into something like a player-reset bundle
    commands.entity(*player).insert((
        LockedAxes::new()
            .lock_translation_x()
            .lock_translation_z()
            .lock_translation_y(),
        LinearVelocity::ZERO,
        AngularVelocity::ZERO,
        Transform::from_translation(player_settings.initial_position),
    ));

    next_state.set(GameState::Ready);

    for ent in pipe_query.iter() {
        commands.entity(ent).despawn();
    }
}

fn scoring(
    mut commands: Commands,
    pipe_query: Query<(Entity, &Transform), (With<PipePair>, Without<Scored>)>,
    scored_pipe_query: Query<(Entity, &Transform), With<Scored>>,
    mut score_info: ResMut<ScoreInfo>,
    mut scored_event: MessageWriter<ScoredEvent>,
) {
    let score_boundary = 0.0;

    for (pipe_entity, pipe_transform) in &pipe_query {
        if pipe_transform.translation.x < score_boundary {
            commands.entity(pipe_entity).insert(Scored);

            score_info.current_score += 1;

            if score_info.current_score > score_info.high_score {
                score_info.high_score = score_info.current_score;
            }

            scored_event.write(ScoredEvent);

            #[cfg(feature = "debugging")]
            println!(
                "PB: {}, Current Score: {}",
                score_info.high_score, score_info.current_score
            );
        }
    }

    for (pipe_entity, pipe_transform) in &scored_pipe_query {
        if pipe_transform.translation.x > score_boundary + 1.0 {
            commands.entity(pipe_entity).remove::<Scored>();
        }
    }
}
