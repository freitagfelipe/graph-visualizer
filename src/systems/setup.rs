use bevy::{core_pipeline::bloom::BloomSettings, prelude::*};

pub fn setup(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                hdr: true,
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 1000.0 - 0.1),
            ..default()
        },
        BloomSettings {
            threshold: 0.9,
            ..default()
        },
    ));
}
