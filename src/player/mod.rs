mod camera;
mod controls;
pub(crate) mod inputs;

use bevy::prelude::*;
use bevy::transform::TransformSystem::TransformPropagate;
use bevy_xpbd_3d::components::{Collider, ExternalImpulse, ExternalTorque, RigidBody};
use bevy_xpbd_3d::PhysicsSet;
use leafwing_input_manager::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<inputs::Action>::default())
            .add_systems(Startup, setup)
            .add_systems(Update, (controls::jump, controls::movement))
            .add_systems(
                PostUpdate,
                camera::follow_player
                    .after(PhysicsSet::Sync)
                    .before(TransformPropagate),
            );
    }
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Player;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Name::new("Player"),
        Player,
        RigidBody::Dynamic,
        ExternalImpulse::new(Vec3::ZERO),
        ExternalTorque::new(Vec3::ZERO),
        Collider::ball(0.5),
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::UVSphere {
                radius: 0.5,
                sectors: 16,
                stacks: 8,
            })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.9).into()),
            transform: Transform::from_xyz(0.0, 4.0, 0.0),
            ..default()
        },
        InputManagerBundle::<inputs::Action> {
            input_map: inputs::create_input_map(),
            ..default()
        },
    ));
}
