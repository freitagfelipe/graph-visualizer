use crate::systems::window;
use bevy::prelude::*;

pub struct WindowPlugin;

impl Plugin for WindowPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(window::switch_screen_mode);
    }
}
