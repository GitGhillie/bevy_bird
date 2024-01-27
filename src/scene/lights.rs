use bevy::prelude::*;

use bevy::pbr::{CascadeShadowConfig, CascadeShadowConfigBuilder};

// This is necessary because of a lack of https://github.com/bevyengine/bevy/pull/8407
// and because shadows are disabled by default when importing from a gltf.
pub(crate) fn replace_added_lights(
    mut added_directional_lights: Query<(Entity, &mut DirectionalLight), Added<DirectionalLight>>,
    mut commands: Commands,
) {
    for (entity, mut light) in added_directional_lights.iter_mut() {
        light.illuminance *= 5.0;
        light.shadows_enabled = true;

        let shadow_config: CascadeShadowConfig = CascadeShadowConfigBuilder {
            first_cascade_far_bound: 15.0,
            maximum_distance: 135.0, //todo check if config is good
            ..default()
        }
        .into();

        commands.entity(entity).insert(shadow_config);
    }
}
