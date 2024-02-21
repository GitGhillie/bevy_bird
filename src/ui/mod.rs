mod input_prompts;
mod score;

use bevy::prelude::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(score::ScoreTextPlugin)
            .add_plugins(input_prompts::PromptPlugin);
    }
}
