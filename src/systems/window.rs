use bevy::prelude::*;

pub fn switch_screen_mode(input: Res<Input<KeyCode>>, mut windows: ResMut<Windows>) {
    if !input.just_released(KeyCode::F11) {
        return;
    }

    let window = windows
        .get_primary_mut()
        .expect("Can not get the primary window");

    if window.mode() == WindowMode::Windowed {
        window.set_mode(WindowMode::BorderlessFullscreen);
    } else {
        window.set_mode(WindowMode::Windowed);
    }
}
