use bevy::prelude::*;

use crate::{
    resources::EdgeSettings,
    systems::{
        edges::{self, CreateOrUnspawnEdgeEvent, RemoveEdgeEvent, UpdateEdgeEvent},
        node,
    },
};

pub struct EdgesPlugin;

impl Plugin for EdgesPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EdgeSettings>()
            .add_event::<UpdateEdgeEvent>()
            .add_event::<RemoveEdgeEvent>()
            .add_event::<CreateOrUnspawnEdgeEvent>()
            .add_system(
                edges::emit_create_or_unspawn_edge_event.before(node::mark_node_to_create_edge),
            )
            .add_system(edges::create_or_unspawn_edge)
            .add_system(
                edges::update_edge_after_moving_node
                    .after(node::fix_off_screen_node_positions)
                    .after(node::emit_update_edge_event_after_node_collision),
            )
            .add_system(edges::remove_edge_after_remove_node);
    }
}
