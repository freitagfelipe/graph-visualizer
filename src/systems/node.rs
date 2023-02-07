use crate::components::{MovingNode, Node};
use crate::resources::{NodeSettings, VisualizerState};
use crate::utils;
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_rapier2d::prelude::*;

fn fix_node_position_if_needed(
    height: f32,
    width: f32,
    mouse_x: f32,
    mouse_y: f32,
    radius: f32,
) -> (f32, f32) {
    let mut new_node_position = (mouse_x, mouse_y);

    if mouse_x + radius >= width / 2.0 {
        new_node_position.0 = width / 2.0 - radius;
    } else if mouse_x - radius <= width * -1.0 / 2.0 {
        new_node_position.0 = (width / 2.0 - radius) * -1.0;
    }

    if mouse_y + radius >= height / 2.0 {
        new_node_position.1 = height / 2.0 - radius;
    } else if mouse_y - radius <= height * -1.0 / 2.0 {
        new_node_position.1 = (height / 2.0 - radius) * -1.0;
    }

    new_node_position
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
            color_material.color = node_settings.moving_color;
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

    if !buttons.pressed(MouseButton::Left) {
        return;
    }

    let window = windows
        .get_primary()
        .expect("Can not get the primary window");

    let Some((x, y)) = utils::get_mouse_coordinates(window) else {
        return;
    };

    for (mut transform, _) in query.iter_mut() {
        transform.translation.x = x;
        transform.translation.y = y;
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

    if !buttons.just_released(MouseButton::Left) {
        return;
    }

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

pub fn fix_off_screen_node_positions(
    mut query: Query<(&mut Transform, With<Node>)>,
    windows: Res<Windows>,
    node_settings: Res<NodeSettings>,
) {
    let window = windows
        .get_primary()
        .expect("Can not get the primary window");

    let height = window.physical_height();
    let width = window.physical_width();

    for (mut transform, _) in query.iter_mut() {
        let (new_node_x, new_node_y) = fix_node_position_if_needed(
            height as f32,
            width as f32,
            transform.translation.x,
            transform.translation.y,
            node_settings.radius,
        );

        transform.translation.x = new_node_x;
        transform.translation.y = new_node_y;
    }
}
