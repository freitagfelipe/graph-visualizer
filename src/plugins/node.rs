use bevy::prelude::*;

use crate::resources::NodeSettings;
use crate::systems::node::{self, ChangeNodeColorEvent};

pub struct NodePlugin;

impl Plugin for NodePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<NodeSettings>()
            .add_event::<ChangeNodeColorEvent>()
            .add_system(node::spawn_node.before(node::unmark_node_that_was_moving))
            .add_system(node::remove_node)
            .add_system(node::mark_node_to_move)
            .add_system(node::move_node)
            .add_system(node::unmark_node_that_was_moving)
            .add_system(node::change_node_color)
            .add_system(
                node::fix_off_screen_node_positions
                    .after(node::spawn_node)
                    .after(node::move_node),
            );
    }
}
