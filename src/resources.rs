use bevy::prelude::{Color, Resource};

#[derive(Resource, Default)]
pub struct VisualizerState {
    pub is_moving_node: bool,
}

#[derive(Resource)]
pub struct NodeSettings {
    pub base_color: Color,
    pub selected_color: Color,
    pub radius: f32,
}

impl Default for NodeSettings {
    fn default() -> Self {
        Self {
            base_color: Color::ALICE_BLUE,
            selected_color: Color::CYAN,
            radius: 12.0,
        }
    }
}
