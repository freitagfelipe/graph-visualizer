use bevy::{app::PluginGroupBuilder, prelude::*};

mod edges;
mod node;
mod setup;
mod window;

pub struct AppPlugins;

impl PluginGroup for AppPlugins {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(setup::SetupPlugin)
            .add(window::WindowPlugin)
            .add(node::NodePlugin)
            .add(edges::EdgesPlugin)
    }
}
