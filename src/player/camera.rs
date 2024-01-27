use crate::player;
use bevy::prelude::*;

pub(crate) fn follow_player(
    player_query: Query<&GlobalTransform, With<player::Player>>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
) {
    let player_transform = player_query.single();
    let mut camera_transform = camera_query.single_mut();

    let camera_offset = Vec3::new(-2.5, 4.5, 9.0);

    camera_transform.translation = player_transform.translation() + camera_offset;

    *camera_transform = camera_transform.looking_at(player_transform.translation(), Vec3::Y);
}
