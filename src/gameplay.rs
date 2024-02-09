use crate::scene::{spawn_level, SceneSettings};
use bevy::prelude::*;
use bevy_xpbd_3d::components::LockedAxes;
use bevy_xpbd_3d::prelude::Collision;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    Ready,
    Playing,
    Dead,
}

pub struct StateTransitionPlugin;

impl Plugin for StateTransitionPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>()
            .add_systems(OnEnter(GameState::Ready), spawn_level)
            .add_systems(OnEnter(GameState::Playing), start_game)
            .add_systems(OnEnter(GameState::Dead), end_game)
            .add_systems(
                PostUpdate,
                crate::player::controls::check_for_game_start.run_if(in_state(GameState::Ready)),
            )
            .add_systems(Update, crate::player::controls::jump) //todo: Sometimes starting the game with jump will not actually jump the player
            .add_systems(
                Update,
                check_for_collisions.run_if(in_state(GameState::Playing)),
            );
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

fn end_game(
    mut player_query: Query<Entity, With<LockedAxes>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut commands: Commands,
    pipe_query: Query<Entity, With<crate::scene::pipes::PipesMarker>>,
) {
    println!("Ending game");

    for player in &mut player_query {
        commands.entity(player).insert(
            LockedAxes::new()
                .lock_translation_x()
                .lock_translation_z()
                .lock_translation_y(),
        );
    }

    next_state.set(GameState::Ready);

    for ent in pipe_query.iter() {
        commands.entity(ent).despawn_recursive();
    }
}
