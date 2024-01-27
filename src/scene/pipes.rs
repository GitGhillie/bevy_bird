use crate::scene::SceneAssets;
use bevy::ecs::system::Command;
use bevy::prelude::*;
use bevy_xpbd_3d::components::{Collider, RigidBody};
use bevy_xpbd_3d::math::PI;

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
pub struct PipesMarker;

pub struct SpawnPipe {
    pub position_x: f32,
}

impl Command for SpawnPipe {
    fn apply(self, world: &mut World) {
        let assets = world.get_resource::<SceneAssets>();

        if let Some(assets) = assets {
            let collider_length = 10.0;
            let pipe_gap_y = 3.0;

            let pipe_handle = assets.pipe.clone_weak();

            let transform_lower = Transform::from_xyz(0.0, 0.0, 0.0);
            let mut transform_upper = Transform::from_xyz(0.0, pipe_gap_y, 0.0);
            transform_upper.rotate_local_z(PI);

            let parent_components = (
                Name::from("PipeParent"),
                PipesMarker,
                VisibilityBundle::default(),
                TransformBundle {
                    local: Transform::from_xyz(self.position_x, 0.0, 0.0),
                    ..default()
                },
            );

            world.spawn(parent_components).with_children(|parent| {
                let transforms = [transform_lower, transform_upper];
                for transform in transforms {
                    let pipe_components = (
                        Name::from("Pipe"),
                        RigidBody::Kinematic,
                        SceneBundle {
                            scene: pipe_handle.clone_weak(),
                            transform,
                            ..default()
                        },
                    );

                    let collider_components = (
                        Collider::cuboid(2.0, collider_length, 2.0),
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
