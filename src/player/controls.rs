use bevy::prelude::*;
use bevy_xpbd_3d::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::player::inputs::Action;

pub fn jump(mut query: Query<(&ActionState<Action>, &mut ExternalImpulse)>) {
    let (action_state, mut impulse) = query.single_mut();

    if action_state.just_pressed(Action::Jump) {
        impulse.y = 5.0;
    }
}

pub fn movement(mut query: Query<(&ActionState<Action>, &mut ExternalTorque)>) {
    let (action_state, mut torque) = query.single_mut();

    let axis_pair = action_state
        .clamped_axis_pair(Action::Move)
        .unwrap()
        .xy()
        .clamp_length_max(1.0);

    torque.x = -axis_pair.y;
    torque.z = -axis_pair.x;
}
