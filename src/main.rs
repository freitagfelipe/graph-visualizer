use bevy::{prelude::*, window::WindowResizeConstraints};
use graph_visualizer::AppPlugins;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "Graph visualizer".to_string(),
                width: 800.0,
                height: 600.0,
                resizable: true,
                resize_constraints: WindowResizeConstraints {
                    min_width: 800.0,
                    min_height: 600.0,
                    ..default()
                },
                ..default()
            },
            ..default()
        }))
        .add_plugins(AppPlugins)
        .run();
}
