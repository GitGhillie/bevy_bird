use crate::scene::{SceneAssets, SceneSettings};
use avian3d::math::PI;
use avian3d::prelude::*;
use bevy::ecs::world::Command;
use bevy::prelude::*;

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
pub struct PipePair;

pub struct SpawnPipePair {
    pub position_x: f32,
    pub rotation: f32,
}

impl Command for SpawnPipePair {
    fn apply(self, world: &mut World) {
        let assets = world.get_resource::<SceneAssets>();
        let scene_settings = world.get_resource::<SceneSettings>().unwrap();

        if let Some(assets) = assets {
            let collider_length = 10.0;

            let pipe_handle = assets.pipe.clone_weak();

            let transform_lower = Transform::from_xyz(0.0, 0.0, 0.0);
            let mut transform_upper = Transform::from_xyz(0.0, scene_settings.pipe_gap_y, 0.0);
            transform_upper.rotate_local_z(PI);

            let mut parent_transform = Transform::from_xyz(self.position_x, 0.0, 0.0);
            parent_transform.rotate_local_y(self.rotation);

            let parent_components = (
                Name::from("PipePair"),
                PipePair,
                Visibility::default(),
                parent_transform,
            );

            world.spawn(parent_components).with_children(|parent| {
                let transforms = [transform_lower, transform_upper];
                for transform in transforms {
                    let pipe_components = (
                        Name::from("Pipe"),
                        RigidBody::Kinematic,
                        SceneRoot(pipe_handle.clone_weak()),
                        transform,
                    );

                    let collider_components = (
                        Collider::cuboid(1.9, collider_length, 1.9),
                        Transform::from_xyz(0.0, -collider_length / 2.0, 0.0),
                    );

                    parent.spawn(pipe_components).with_children(|parent| {
                        parent.spawn(collider_components);
                    });
                }
            });
        }
    }
}
