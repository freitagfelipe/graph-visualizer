use crate::components::{MovingNode, Node, SelectedNode};
use crate::resources::{NodeSettings, VisualizerState};
use crate::utils;
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_rapier2d::prelude::*;

use super::edges::{RemoveEdgeEvent, UpdateEdgeEvent};

pub struct ChangeNodeColorEvent {
    pub entity: Entity,
    pub color: Color,
}

fn fix_node_position_if_needed(
    height: f32,
    width: f32,
    node_x: f32,
    node_y: f32,
    radius: f32,
) -> Option<(f32, f32)> {
    let mut x = node_x;
    let mut y = node_y;

    if node_x + radius >= width / 2.0 {
        x = width / 2.0 - radius;
    } else if node_x - radius <= width * -1.0 / 2.0 {
        x = (width / 2.0 - radius) * -1.0;
    }

    if node_y + radius >= height / 2.0 {
        y = height / 2.0 - radius;
    } else if node_y - radius <= height * -1.0 / 2.0 {
        y = (height / 2.0 - radius) * -1.0;
    }

    if x != node_x || y != node_y {
        return Some((x, y));
    }

    None
}

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

    if !buttons.just_released(MouseButton::Left) {
        return;
    }

    let window = windows
        .get_primary()
        .expect("Can not get the primary window");

    let Some((x, y)) = utils::get_mouse_coordinates(window) else {
        return;
    };

    commands.spawn((
        RigidBody::Dynamic,
        Collider::ball(node_settings.radius),
        GravityScale(0.0),
        Damping {
            linear_damping: 20.0,
            ..default()
        },
        Velocity {
            linvel: Vec2::ZERO,
            ..default()
        },
        MaterialMesh2dBundle {
            mesh: meshes
                .add(shape::Circle::new(node_settings.radius).into())
                .into(),
            material: materials.add(ColorMaterial::from(node_settings.base_color)),
            transform: Transform::from_translation(Vec3::new(x, y, 1.0)),
            ..default()
        },
        Node,
    ));
}

pub fn remove_node(
    mut commands: Commands,
    query: Query<(Entity, &Transform), With<Node>>,
    mut event_writer: EventWriter<RemoveEdgeEvent>,
    buttons: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    node_settings: Res<NodeSettings>,
    visualizer_state: ResMut<VisualizerState>,
) {
    if visualizer_state.is_moving_node {
        return;
    }

    if !buttons.just_released(MouseButton::Right) {
        return;
    }

    let window = windows
        .get_primary()
        .expect("Can not get the primary window");

    let Some((x, y)) = utils::get_mouse_coordinates(window) else {
        return;
    };

    let mut entity_to_despawn = None;

    for (entity, transform) in query.iter() {
        if utils::is_mouse_on_node(
            x,
            y,
            transform.translation.x,
            transform.translation.y,
            node_settings.radius,
        ) {
            entity_to_despawn = Some(entity);

            break;
        }
    }

    if let Some(entity_to_despawn) = entity_to_despawn {
        commands.entity(entity_to_despawn).despawn();

        event_writer.send(RemoveEdgeEvent {
            removed_node: entity_to_despawn,
        });
    }
}

pub fn mark_node_to_move(
    mut commands: Commands,
    query: Query<(Entity, &Transform), With<Node>>,
    mut event_writer: EventWriter<ChangeNodeColorEvent>,
    buttons: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    node_settings: Res<NodeSettings>,
    visualizer_state: Res<VisualizerState>,
) {
    if visualizer_state.is_moving_node {
        return;
    }

    if !buttons.pressed(MouseButton::Left) {
        return;
    }

    let window = windows
        .get_primary()
        .expect("Can not get the primary window");

    let Some((x, y)) = utils::get_mouse_coordinates(window) else {
        return;
    };

    let mut node_to_mark = None;

    for (entity, transform) in query.iter() {
        if utils::is_mouse_on_node(
            x,
            y,
            transform.translation.x,
            transform.translation.y,
            node_settings.radius,
        ) {
            node_to_mark = Some(entity);

            break;
        }
    }

    if let Some(entity) = node_to_mark {
        commands.entity(entity).insert(MovingNode);

        commands.insert_resource(VisualizerState {
            is_moving_node: true,
        });

        event_writer.send(ChangeNodeColorEvent {
            entity,
            color: node_settings.moving_color,
        });
    }
}

pub fn move_node(
    mut query: Query<(Entity, &mut Transform), With<MovingNode>>,
    mut event_writer: EventWriter<UpdateEdgeEvent>,
    windows: Res<Windows>,
    visualizer_state: Res<VisualizerState>,
) {
    if !visualizer_state.is_moving_node {
        return;
    }

    let window = windows
        .get_primary()
        .expect("Can not get the primary window");

    let Some((x, y)) = utils::get_mouse_coordinates(window) else {
        return;
    };

    let (entity, mut transform) = query
        .get_single_mut()
        .expect("Move node: no moving entity or more than one");

    transform.translation.x = x;
    transform.translation.y = y;

    event_writer.send(UpdateEdgeEvent {
        changed_node: entity,
        transform: *transform,
    });
}

pub fn unmark_node_that_was_moving(
    mut commands: Commands,
    query: Query<(Entity, Option<&SelectedNode>), With<MovingNode>>,
    mut event_writer: EventWriter<ChangeNodeColorEvent>,
    buttons: Res<Input<MouseButton>>,
    node_settings: Res<NodeSettings>,
    visualizer_state: Res<VisualizerState>,
) {
    if !visualizer_state.is_moving_node {
        return;
    }

    if !buttons.just_released(MouseButton::Left) {
        return;
    }

    let (entity, selected_node) = query
        .get_single()
        .expect("Unmark node that was moving: no moving entity or more than one");

    commands.insert_resource(VisualizerState {
        is_moving_node: false,
    });

    commands.entity(entity).remove::<MovingNode>();

    event_writer.send(ChangeNodeColorEvent {
        entity,
        color: match selected_node {
            Some(_) => node_settings.selected_color,
            None => node_settings.base_color,
        },
    });
}

pub fn mark_node_to_create_edge(
    mut commands: Commands,
    query: Query<(Entity, &Transform, Option<&SelectedNode>), With<Node>>,
    mut event_writer: EventWriter<ChangeNodeColorEvent>,
    windows: Res<Windows>,
    buttons: Res<Input<MouseButton>>,
    node_settings: Res<NodeSettings>,
    visualizer_state: Res<VisualizerState>,
) {
    if visualizer_state.is_moving_node {
        return;
    }

    if !buttons.just_released(MouseButton::Middle) {
        return;
    }

    let window = windows
        .get_primary()
        .expect("Can not get the primary window");

    let Some((x, y)) = utils::get_mouse_coordinates(window) else {
        return;
    };

    for (entity, transform, selected_node) in query.iter() {
        if !utils::is_mouse_on_node(
            x,
            y,
            transform.translation.x,
            transform.translation.y,
            node_settings.radius,
        ) {
            continue;
        }

        if selected_node.is_some() {
            commands.entity(entity).remove::<SelectedNode>();
        } else {
            commands.entity(entity).insert(SelectedNode);
        }

        event_writer.send(ChangeNodeColorEvent {
            entity,
            color: match selected_node {
                Some(_) => node_settings.base_color,
                None => node_settings.selected_color,
            },
        });

        break;
    }
}

pub fn fix_off_screen_node_positions(
    mut query: Query<(Entity, &mut Transform), With<Node>>,
    mut event_writer: EventWriter<UpdateEdgeEvent>,
    windows: Res<Windows>,
    node_settings: Res<NodeSettings>,
) {
    let window = windows
        .get_primary()
        .expect("Can not get the primary window");

    let height = window.physical_height();
    let width = window.physical_width();

    for (entity, mut transform) in query.iter_mut() {
        let Some((new_x, new_y)) = fix_node_position_if_needed(
            height as f32,
            width as f32,
            transform.translation.x,
            transform.translation.y,
            node_settings.radius,
        ) else {
            continue;
        };

        transform.translation.x = new_x;
        transform.translation.y = new_y;

        event_writer.send(UpdateEdgeEvent {
            changed_node: entity,
            transform: *transform,
        });
    }
}

pub fn change_node_color(
    mut commands: Commands,
    mut event_reader: EventReader<ChangeNodeColorEvent>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for ev in event_reader.iter() {
        commands
            .entity(ev.entity)
            .insert(materials.add(ColorMaterial::from(ev.color)));
    }
}

pub fn emit_update_edge_event_after_node_collision(
    query: Query<(Entity, &Velocity, &Transform), With<Node>>,
    mut event_writer: EventWriter<UpdateEdgeEvent>,
) {
    for (entity, velocity, transform) in query.iter() {
        if velocity.linvel != Vec2::ZERO {
            event_writer.send(UpdateEdgeEvent {
                changed_node: entity,
                transform: *transform,
            });
        }
    }
}
