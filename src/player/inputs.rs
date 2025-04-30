use bevy::prelude::{GamepadButton, KeyCode, MouseButton, Reflect};
use leafwing_input_manager::prelude::*;

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub(crate) enum Action {
    Jump,
}

// Stores "which actions are currently activated"
pub(crate) fn create_input_map() -> InputMap<Action> {
    let mut input_map = InputMap::default();

    input_map.insert(Action::Jump, KeyCode::Space);
    input_map.insert(Action::Jump, MouseButton::Left);
    input_map.insert(Action::Jump, GamepadButton::South);

    input_map
}
