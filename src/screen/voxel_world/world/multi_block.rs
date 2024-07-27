use bevy::prelude::*;

use crate::screen::voxel_world::BlockType;

#[derive(Component)]
struct MultiBlock {
    size: IVec3,
    origin: IVec3,
    building_blocks: Vec<BlockType>,
}
