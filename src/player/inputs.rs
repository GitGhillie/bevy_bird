use bevy::prelude::{KeyCode, Reflect};
use leafwing_input_manager::axislike::{DualAxis, VirtualDPad};
use leafwing_input_manager::input_map::InputMap;
use leafwing_input_manager::Actionlike;

// This is the list of "things in the game I want to be able to do based on input"
#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub(crate) enum Action {
    Move,
    Jump,
}

// Stores "which actions are currently activated"
// Map some arbitrary keys into a virtual direction pad that triggers our move action
pub(crate) fn create_input_map() -> InputMap<Action> {
    let mut input_map = InputMap::default();

    input_map.insert(KeyCode::Space, Action::Jump);
    input_map.insert(DualAxis::left_stick(), Action::Move);
    input_map.insert(
        VirtualDPad {
            up: KeyCode::W.into(),
            down: KeyCode::S.into(),
            left: KeyCode::A.into(),
            right: KeyCode::D.into(),
        },
        Action::Move,
    );

    input_map
}
