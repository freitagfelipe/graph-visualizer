use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::{
    components::{NeighborNodes, SelectedNode},
    resources::{EdgeSettings, NodeSettings},
};

use super::node::ChangeNodeColorEvent;

pub struct UpdateEdgeEvent {
    pub changed_node: Entity,
    pub transform: Transform,
}

pub struct RemoveEdgeEvent {
    pub removed_node: Entity,
}

pub fn create_edge(
    mut commands: Commands,
    query: Query<(Entity, &Transform, With<SelectedNode>)>,
    mut event_writer: EventWriter<ChangeNodeColorEvent>,
    edge_settings: Res<EdgeSettings>,
    node_settings: Res<NodeSettings>,
) {
    if query.iter().size_hint().0 < 2 {
        return;
    }

    let mut iter = query.iter();

    let (first_entity, start_pos, _) = iter.next().expect("Can not get the first selected node");
    let (second_entity, end_pos, _) = iter.next().expect("Can not get the second selected node");

    let mut path_builder = PathBuilder::new();

    path_builder.move_to(Vec2::new(start_pos.translation.x, start_pos.translation.y));
    path_builder.line_to(Vec2::new(end_pos.translation.x, end_pos.translation.y));

    let line = path_builder.build();

    commands.entity(first_entity).remove::<SelectedNode>();
    commands.entity(second_entity).remove::<SelectedNode>();

    event_writer.send(ChangeNodeColorEvent {
        entity: first_entity,
        color: node_settings.base_color,
    });

    event_writer.send(ChangeNodeColorEvent {
        entity: second_entity,
        color: node_settings.base_color,
    });

    commands.spawn((
        NeighborNodes {
            v: first_entity,
            u: second_entity,
            pos_v: *start_pos,
            pos_u: *end_pos,
        },
        GeometryBuilder::build_as(
            &line,
            DrawMode::Stroke(StrokeMode::new(edge_settings.color, edge_settings.size)),
            Transform::default(),
        ),
    ));
}

pub fn update_edge_after_moving_node(
    mut query: Query<(&mut Path, &mut NeighborNodes)>,
    mut event_reader: EventReader<UpdateEdgeEvent>,
) {
    for ev in event_reader.iter() {
        let (changed_node, translation) = (ev.changed_node, ev.transform.translation);

        let mut path_builder = PathBuilder::new();

        path_builder.move_to(Vec2::new(translation.x, translation.y));

        for (mut path, mut neighbor_nodes) in query.iter_mut() {
            let mut end_pos = None;

            if neighbor_nodes.v == changed_node {
                end_pos = Some(neighbor_nodes.pos_u);

                neighbor_nodes.pos_v = ev.transform;
            } else if neighbor_nodes.u == changed_node {
                end_pos = Some(neighbor_nodes.pos_v);

                neighbor_nodes.pos_u = ev.transform;
            }

            if let Some(end_pos) = end_pos {
                path_builder.line_to(Vec2::new(end_pos.translation.x, end_pos.translation.y));

                *path = path_builder.build();

                path_builder = PathBuilder::new();

                path_builder.move_to(Vec2::new(translation.x, translation.y));
            }
        }
    }
}

pub fn remove_edge(
    mut commands: Commands,
    query: Query<(Entity, &NeighborNodes)>,
    mut event_reader: EventReader<RemoveEdgeEvent>,
) {
    for ev in event_reader.iter() {
        let removed_node = ev.removed_node;

        for (entity, neighbor_nodes) in query.iter() {
            if neighbor_nodes.v == removed_node || neighbor_nodes.u == removed_node {
                commands.entity(entity).despawn();
            }
        }
    }
}
