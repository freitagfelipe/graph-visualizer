use crate::components::{MovingNode, Node};
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

pub fn remove_node(
    mut commands: Commands,
    query: Query<(Entity, &Transform, With<Node>)>,
    buttons: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    node_settings: Res<NodeSettings>,
    visualizer_state: ResMut<VisualizerState>,
) {
    if visualizer_state.is_moving_node {
        return;
    }

    if buttons.just_released(MouseButton::Right) {
        let window = windows
            .get_primary()
            .expect("Can not get the primary window");

        if let Some((x, y)) = utils::get_mouse_coordinates(window) {
            let mut entity_to_despawn = None;

            for (entity, transform, _) in query.iter() {
                if utils::is_mouse_on_node(
                    x,
                    y,
                    transform.translation.x,
                    transform.translation.y,
                    node_settings.radius,
                ) {
                    entity_to_despawn = Some(entity);
                }
            }

            if let Some(entity_to_despawn) = entity_to_despawn {
                commands.entity(entity_to_despawn).despawn();
            }
        }
    }
}

pub fn mark_node_to_move(
    mut commands: Commands,
    query: Query<(Entity, &Transform, &Handle<ColorMaterial>)>,
    buttons: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    node_settings: Res<NodeSettings>,
    mut visualizer_state: ResMut<VisualizerState>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if visualizer_state.is_moving_node {
        return;
    }

    if buttons.pressed(MouseButton::Left) {
        let window = windows
            .get_primary()
            .expect("Can not get the primary window");

        if let Some((x, y)) = utils::get_mouse_coordinates(window) {
            let mut node_to_mark = None;

            for (entity, transform, color_material) in query.iter() {
                if utils::is_mouse_on_node(
                    x,
                    y,
                    transform.translation.x,
                    transform.translation.y,
                    node_settings.radius,
                ) {
                    node_to_mark = Some((entity, color_material));
                }
            }

            if let Some((entity, color_material)) = node_to_mark {
                commands.entity(entity).insert(MovingNode);
                visualizer_state.is_moving_node = true;

                if let Some(mut color_material) = materials.get_mut(color_material) {
                    color_material.color = node_settings.selected_color;
                }
            }
        }
    }
}

pub fn move_node(
    mut query: Query<(&mut Transform, With<MovingNode>)>,
    buttons: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    visualizer_state: Res<VisualizerState>,
) {
    if !visualizer_state.is_moving_node {
        return;
    }

    if buttons.pressed(MouseButton::Left) {
        let window = windows
            .get_primary()
            .expect("Can not get the primary window");

        if let Some((x, y)) = utils::get_mouse_coordinates(window) {
            for (mut transform, _) in query.iter_mut() {
                transform.translation.x = x;
                transform.translation.y = y;
            }
        }
    }
}

pub fn unmark_node_that_was_moving(
    mut commands: Commands,
    query: Query<(Entity, &Handle<ColorMaterial>, With<MovingNode>)>,
    buttons: Res<Input<MouseButton>>,
    node_settings: Res<NodeSettings>,
    mut visualizer_state: ResMut<VisualizerState>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if !visualizer_state.is_moving_node {
        return;
    }

    if buttons.just_released(MouseButton::Left) {
        visualizer_state.is_moving_node = false;

        let (entity, color_material, _) = query
            .iter()
            .next()
            .expect("Can not get the node that was moving");

        commands.entity(entity).remove::<MovingNode>();

        if let Some(mut color_material) = materials.get_mut(color_material) {
            color_material.color = node_settings.base_color;
        }
    }
}
