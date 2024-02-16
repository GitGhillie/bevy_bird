pub(crate) mod controls;
pub(crate) mod inputs;

use crate::gameplay::JumpedEvent;
use bevy::pbr::NotShadowCaster;
use bevy::prelude::*;
use bevy_kira_audio::Audio;
use bevy_xpbd_3d::components::{Collider, RigidBody};
use bevy_xpbd_3d::prelude::*;
use leafwing_input_manager::prelude::*;

#[derive(Reflect, Resource, Default)]
#[reflect(Resource)]
pub struct PlayerSettings {
    pub initial_position: Vec3,
    pub jump_velocity: f32,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<inputs::Action>::default())
            .register_type::<PlayerSettings>()
            .insert_resource(PlayerSettings {
                jump_velocity: 10.0,
                initial_position: Vec3::new(0.0, 1.0, 0.0),
            })
            .add_systems(Startup, setup)
            .add_systems(Update, gunshot_light);
    }
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Player;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    player_settings: Res<PlayerSettings>,
) {
    let parent = commands
        .spawn((
            Name::new("Player"),
            Player,
            RigidBody::Dynamic,
            GravityScale(4.0),
            LockedAxes::new()
                .lock_translation_x()
                .lock_translation_z()
                .lock_translation_y(),
            LinearVelocity::ZERO,
            Collider::ball(0.5),
            SceneBundle {
                scene: asset_server.load("objects/bird.glb#Scene0"),
                transform: Transform::from_translation(player_settings.initial_position),
                ..default()
            },
            InputManagerBundle::<inputs::Action> {
                input_map: inputs::create_input_map(),
                ..default()
            },
        ))
        .id();

    let light1 = commands
        .spawn(PointLightBundle {
            point_light: PointLight {
                intensity: 0.0,
                color: Color::ORANGE,
                shadows_enabled: true,
                ..default()
            },
            transform: Transform::from_xyz(0.4, 0.0, 0.0),
            ..default()
        })
        .id();

    let light2 = commands
        .spawn(PointLightBundle {
            point_light: PointLight {
                intensity: 0.0,
                color: Color::ORANGE,
                shadows_enabled: true,
                ..default()
            },
            transform: Transform::from_xyz(0.05, -0.81, 0.0),
            ..default()
        })
        .id();

    commands.entity(parent).push_children(&[light1, light2]);
}

fn gunshot_light(
    mut light_query: Query<&mut PointLight>,
    mut jump_event: EventReader<JumpedEvent>,
) {
    let gunshot_event = !jump_event.is_empty();
    for _ in jump_event.read() {} // Clear the queue

    for mut light in &mut light_query {
        light.intensity -= 90.0;

        if light.intensity < 0.0 {
            light.intensity = 0.0;
        }

        if gunshot_event {
            light.intensity = 1000.0;
        }
    }
}
