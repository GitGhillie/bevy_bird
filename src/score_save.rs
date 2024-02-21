use crate::gameplay::{GameState, ScoreInfo};
use bevy::prelude::*;
use std::io::{Read, Write};

const PATH: &str = "./file.ron";

pub struct SavePlugin;

impl Plugin for SavePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_high_score)
            .add_systems(OnEnter(GameState::Dead), save_high_score);
    }
}

fn load_high_score(mut score_info: ResMut<ScoreInfo>) {
    if let Ok(mut file) = std::fs::File::open(PATH) {
        let mut data_buffer = String::default();

        file.read_to_string(&mut data_buffer)
            .expect("Could not read ron file");

        if let Ok(data) = ron::from_str(&data_buffer) {
            *score_info = data;
        }
    } else {
        std::fs::File::create(PATH).unwrap();
    }
}

fn save_high_score(score_info: Res<ScoreInfo>) {
    let score_copy = *score_info;
    let data = ron::to_string(&score_copy).unwrap();

    // write data to file
    let mut file = std::fs::File::create(PATH).unwrap();
    file.write_all(data.as_bytes()).unwrap();
}
