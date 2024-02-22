pub(crate) mod controls;
pub(crate) mod inputs;

use crate::gameplay::{GameState, JumpedEvent};
use bevy::prelude::*;
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
            .add_systems(OnEnter(GameState::Ready), setup)
            .add_systems(Update, gunshot_lighting);
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
            Collider::capsule(0.45, 0.3),
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

fn gunshot_lighting(
    mut light_query: Query<&mut PointLight>,
    mut jump_event: EventReader<JumpedEvent>,
) {
    let gunshot_event = !jump_event.is_empty();
    for _ in jump_event.read() {} // Clear the queue

    for mut light in &mut light_query {
        light.intensity -= 50_000.0;

        if light.intensity < 0.0 {
            light.intensity = 0.0;
        }

        if gunshot_event {
            light.intensity = 1_000_000.0;
        }
    }
}
