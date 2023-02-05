use bevy::window::Window;

pub fn get_mouse_coordinates(window: &Window) -> Option<(f32, f32)> {
    let mouse_position = window.cursor_position()?;
    let (window_center_x, window_center_y) = (window.width() / 2.0, window.height() / 2.0);

    Some((
        mouse_position.x - window_center_x,
        mouse_position.y - window_center_y,
    ))
}
