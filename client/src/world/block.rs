use bevy::prelude::*;

#[derive(Component)]
pub struct BlockQuadMesh;

#[derive(Component)]
pub struct BlockId(pub String);

#[derive(Component)]
pub struct BlockPosition(pub Vec3);

#[derive(Bundle)]
pub struct BlockBundle {
    id: BlockId,
    position: BlockPosition,
}

impl BlockBundle {
    pub fn new(position: Vec3) -> Self {
        Self {
            id: BlockId("test".into()),
            position: BlockPosition(position),
        }
    }
}
