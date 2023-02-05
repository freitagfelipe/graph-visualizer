use crate::components::Node;
use crate::resources::{NodeSettings, VisualizerState};
use crate::utils;
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_rapier2d::prelude::*;

pub fn spawn_node(
    mut commands: Commands,
    buttons: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    node_settings: Res<NodeSettings>,
    visualizer_state: Res<VisualizerState>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if visualizer_state.is_moving_node {
        return;
    }

    if buttons.just_released(MouseButton::Left) {
        let window = windows
            .get_primary()
            .expect("Can not get the primary window");

        if let Some((x, y)) = utils::get_mouse_coordinates(window) {
            commands.spawn((
                RigidBody::Dynamic,
                Collider::ball(node_settings.radius),
                GravityScale(0.0),
                Damping {
                    linear_damping: 4.0,
                    angular_damping: 4.0,
                },
                Restitution::coefficient(0.5),
                MaterialMesh2dBundle {
                    mesh: meshes
                        .add(shape::Circle::new(node_settings.radius).into())
                        .into(),
                    material: materials.add(ColorMaterial::from(node_settings.base_color)),
                    transform: Transform::from_translation(Vec3::new(x, y, 0.0)),
                    ..default()
                },
                Node,
            ));
        }
    }
}
