use bevy::prelude::*;
use bevy_xpbd_3d::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::gameplay::GameState;
use crate::player::inputs::Action;
use crate::player::PlayerSettings;

pub fn jump(
    mut query: Query<(&ActionState<Action>, &mut LinearVelocity)>,
    player_settings: Res<PlayerSettings>,
) {
    let (action_state, mut velocity) = query.single_mut();

    if action_state.just_pressed(Action::Jump) {
        velocity.y = player_settings.jump_velocity;
    }
}

pub fn check_for_game_start(
    query: Query<&ActionState<Action>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let action_state = query.single();

    if action_state.just_pressed(Action::Jump) {
        println!("Starting game after Jump");
        next_state.set(GameState::Playing);
    }
}
