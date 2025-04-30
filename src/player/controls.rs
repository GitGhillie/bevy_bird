use avian3d::prelude::*;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::gameplay::{GameState, JumpedEvent};
use crate::player::PlayerSettings;
use crate::player::inputs::Action;

pub fn jump(
    query: Single<(&ActionState<Action>, &mut LinearVelocity)>,
    player_settings: Res<PlayerSettings>,
    mut jumped_event: EventWriter<JumpedEvent>,
) {
    let (action_state, mut velocity) = query.into_inner();

    if action_state.just_pressed(&Action::Jump) {
        velocity.y = player_settings.jump_velocity;
        jumped_event.write(JumpedEvent);
    }
}

pub fn check_for_game_start(
    action_state: Single<&ActionState<Action>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if action_state.just_pressed(&Action::Jump) {
        next_state.set(GameState::Playing);
    }
}
