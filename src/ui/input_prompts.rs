use crate::gameplay::GameState;
use bevy::prelude::*;

pub struct PromptPlugin;

impl Plugin for PromptPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(OnEnter(GameState::Playing), hide)
            .add_systems(OnEnter(GameState::Ready), show);
    }
}

#[derive(Component)]
struct IconParent;

fn hide(mut icon_parents: Query<&mut Visibility, With<IconParent>>) {
    for mut icon_parent in &mut icon_parents {
        *icon_parent = Visibility::Hidden;
    }
}

fn show(mut icon_parents: Query<&mut Visibility, With<IconParent>>) {
    for mut icon_parent in &mut icon_parents {
        *icon_parent = Visibility::Visible;
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut icon_margin = UiRect::all(Val::Px(5.));
    icon_margin.top = Val::VMin(90.);

    let icon_node = Node {
        width: Val::Px(30.0),
        height: Val::Px(30.0),
        margin: icon_margin,
        ..default()
    };

    let icon_node_space = Node {
        width: Val::Px(90.0),
        height: Val::Px(30.0),
        margin: icon_margin,
        ..default()
    };

    commands
        .spawn((
            NodeBundle {
                node: Node {
                    width: Val::Percent(100.0),
                    position_type: PositionType::Absolute,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::FlexStart,
                    ..default()
                },
                ..default()
            },
            IconParent,
        ))
        .with_children(|parent| {
            parent.spawn((
                NodeBundle {
                    node: icon_node.clone(),
                    ..default()
                },
                ImageNode::new(asset_server.load("textures/prompts/left_mouse_button_light.png")),
            ));

            parent.spawn((
                NodeBundle {
                    node: icon_node.clone(),
                    ..default()
                },
                ImageNode::new(asset_server.load("textures/prompts/xbox_a_green.png")),
            ));

            parent.spawn((
                NodeBundle {
                    node: icon_node_space.clone(),
                    ..default()
                },
                ImageNode::new(asset_server.load("textures/prompts/space_light.png")),
            ));
        });
}
