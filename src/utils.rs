use bevy::window::Window;

pub fn get_mouse_coordinates(window: &Window) -> Option<(f32, f32)> {
    let mouse_position = window.cursor_position()?;
    let (window_center_x, window_center_y) = (window.width() / 2.0, window.height() / 2.0);

    Some((
        mouse_position.x - window_center_x,
        mouse_position.y - window_center_y,
    ))
}

pub fn is_mouse_on_node(mouse_x: f32, mouse_y: f32, node_x: f32, node_y: f32, radius: f32) -> bool {
    let distance_between_node_and_mouse =
        ((mouse_x - node_x).powi(2) + (mouse_y - node_y).powi(2)).sqrt();

    distance_between_node_and_mouse <= radius
}
