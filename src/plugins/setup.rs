use bevy::prelude::*;
use bevy_prototype_lyon::prelude::ShapePlugin;
use bevy_rapier2d::prelude::{NoUserData, RapierPhysicsPlugin};

use crate::resources::VisualizerState;
use crate::systems::setup;

pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<VisualizerState>()
            .init_resource::<Msaa>()
            .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(1.0))
            .add_plugin(ShapePlugin)
            .add_startup_system(setup::setup);
    }
}
