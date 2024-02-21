use bevy::prelude::{GamepadButtonType, KeyCode, MouseButton, Reflect};
use leafwing_input_manager::input_map::InputMap;
use leafwing_input_manager::Actionlike;

// This is the list of "things in the game I want to be able to do based on input"
#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub(crate) enum Action {
    Jump,
}

// Stores "which actions are currently activated"
// Map some arbitrary keys into a virtual direction pad that triggers our move action
pub(crate) fn create_input_map() -> InputMap<Action> {
    let mut input_map = InputMap::default();

    input_map.insert(Action::Jump, KeyCode::Space);
    input_map.insert(Action::Jump, MouseButton::Left);
    input_map.insert(Action::Jump, GamepadButtonType::South);

    input_map
}
