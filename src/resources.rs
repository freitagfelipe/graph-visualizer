use bevy::prelude::{Color, Resource};

#[derive(Resource, Default)]
pub struct VisualizerState {
    pub is_moving_node: bool,
}

#[derive(Resource)]
pub struct NodeSettings {
    pub base_color: Color,
    pub selected_color: Color,
    pub moving_color: Color,
    pub radius: f32,
}

impl Default for NodeSettings {
    fn default() -> Self {
        Self {
            base_color: Color::ALICE_BLUE,
            selected_color: Color::CYAN,
            moving_color: Color::PINK,
            radius: 12.0,
        }
    }
}

#[derive(Resource)]
pub struct EdgeSettings {
    pub color: Color,
    pub size: f32,
}

impl Default for EdgeSettings {
    fn default() -> Self {
        Self {
            color: Color::ALICE_BLUE,
            size: 3.5,
        }
    }
}
