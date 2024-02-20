use crate::player::PlayerSettings;
use crate::scene::pipes::PipePair;
use crate::scene::{spawn_level, SceneSettings};
use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
use bevy_xpbd_3d::components::{LinearVelocity, LockedAxes};
use bevy_xpbd_3d::prelude::Collision;
use std::time::Duration;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    Ready,
    Playing,
    Dead,
}

#[derive(Resource, Default)]
pub struct ScoreInfo {
    pub current_score: u32,
    pub high_score: u32,
}

// Indicates if a pipe has passed the player
#[derive(Component)]
struct Scored;

#[derive(Event)]
pub struct ScoredEvent;

#[derive(Event)]
pub struct JumpedEvent;

pub struct StateTransitionPlugin;

impl Plugin for StateTransitionPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>()
            .insert_resource(ScoreInfo::default())
            .add_event::<ScoredEvent>()
            .add_event::<JumpedEvent>()
            .add_systems(OnEnter(GameState::Ready), spawn_level)
            .add_systems(OnEnter(GameState::Playing), start_game)
            .add_systems(
                Update,
                end_game
                    .run_if(in_state(GameState::Dead).and_then(on_timer(Duration::from_secs(1)))),
            )
            .add_systems(
                Update,
                crate::player::controls::check_for_game_start.run_if(in_state(GameState::Ready)),
            )
            .add_systems(
                Update,
                crate::player::controls::jump
                    .run_if(in_state(GameState::Playing))
                    .after(crate::player::controls::check_for_game_start),
            )
            .add_systems(
                Update,
                (check_for_collisions, ramp_up_speed).run_if(in_state(GameState::Playing)),
            )
            .add_systems(Update, scoring);
    }
}

fn start_game(
    mut commands: Commands,
    mut player_query: Query<(Entity, &mut LinearVelocity), With<LockedAxes>>,
    mut scene_settings: ResMut<SceneSettings>,
    player_settings: Res<PlayerSettings>,
    mut score_info: ResMut<ScoreInfo>,
) {
    scene_settings.pipe_speed = 5.0;
    score_info.current_score = 0;

    for (player, mut velocity) in &mut player_query {
        commands.entity(player).insert(
            LockedAxes::new()
                .lock_translation_x()
                .lock_translation_z()
                .lock_rotation_x()
                .lock_rotation_y()
                .lock_rotation_z(),
        );

        // We need to jump when starting the game since the jump action is 'used up' when
        // checking for the state transition from `Ready` to `Playing`.
        velocity.y = player_settings.jump_velocity;
    }
}

fn ramp_up_speed(mut scene_settings: ResMut<SceneSettings>, time: Res<Time>) {
    if scene_settings.pipe_speed < 10.0 {
        scene_settings.pipe_speed += 0.4 * time.delta_seconds();
    }
}

fn check_for_collisions(
    mut collision_event_reader: EventReader<Collision>,
    mut scene_settings: ResMut<SceneSettings>,
    mut next_state: ResMut<NextState<GameState>>,
    mut player_query: Query<Entity, With<LockedAxes>>,
    mut commands: Commands,
) {
    if !collision_event_reader.is_empty() {
        // Drain events so they don't cause issues later
        for _ in collision_event_reader.read() {}

        scene_settings.pipe_speed = 0.0;

        for player in &mut player_query {
            commands.entity(player).insert((LockedAxes::new(),));
        }

        next_state.set(GameState::Dead);
    }
}

fn end_game(
    mut player_query: Query<Entity, With<LockedAxes>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut commands: Commands,
    pipe_query: Query<Entity, With<PipePair>>,
    player_settings: Res<PlayerSettings>,
) {
    for player in &mut player_query {
        commands.entity(player).insert((
            LockedAxes::new()
                .lock_translation_x()
                .lock_translation_z()
                .lock_translation_y()
                .lock_rotation_x()
                .lock_rotation_y()
                .lock_rotation_z(),
            Transform::from_translation(player_settings.initial_position),
        ));
    }

    next_state.set(GameState::Ready);

    for ent in pipe_query.iter() {
        commands.entity(ent).despawn_recursive();
    }
}

fn scoring(
    mut commands: Commands,
    pipe_query: Query<(Entity, &Transform), (With<PipePair>, Without<Scored>)>,
    scored_pipe_query: Query<(Entity, &Transform), With<Scored>>,
    mut score_info: ResMut<ScoreInfo>,
    mut scored_event: EventWriter<ScoredEvent>,
) {
    let score_boundary = 0.0;

    for (pipe_entity, pipe_transform) in &pipe_query {
        if pipe_transform.translation.x < score_boundary {
            commands.entity(pipe_entity).insert(Scored);

            score_info.current_score += 1;

            if score_info.current_score > score_info.high_score {
                score_info.high_score = score_info.current_score;
            }

            scored_event.send(ScoredEvent);

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
