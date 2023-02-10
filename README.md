# Graph visualizer

- The Graph Visualizer is a tool designed to serve as a foundation for other graph-based visualizers that I plan to create. It's versatile and can be used for personal projects. If you have any questions or need clarification, feel free to reach out to me for assistance.

## How the visualizer was made

- The graph visualizer is written in Rust, using [Bevy](https://crates.io/crates/bevy) as the engine and other crates like:
    - [bevy_prototype_lyon](https://crates.io/crates/bevy_prototype_lyon)
    - [bevy_rapier2d](https://crates.io/crates/bevy_rapier2d)

## Usage help

- You can create nodes by clicking the left mouse button, and delete them by clicking the right mouse button. To create an edge between two nodes, simply click on two nodes with the middle mouse button. If you click on two nodes that already have an edge between them, the edge will be deleted. To toggle between fullscreen and windowed mode, simply press the F11 key.

## Projects using the graph visualizer

- Visualizing Breadth-first Search and Depth-first Search algorithms (coming soon)
- Visualizing Minimal Spannig Tree (MST) (coming soon)
- Visualizing Dijkstra's Shortest Path algorithm (coming soon)