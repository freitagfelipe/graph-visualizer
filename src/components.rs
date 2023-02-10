use bevy::prelude::*;

#[derive(Component)]
pub struct Node;

#[derive(Component)]
pub struct MovingNode;

#[derive(Component)]
pub struct SelectedNode;

#[derive(Component)]
pub struct NeighborNodes {
    pub v: Entity,
    pub u: Entity,
    pub pos_v: Transform,
    pub pos_u: Transform,
}
