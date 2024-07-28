use bevy::prelude::*;

use crate::{
    game::HexSelect,
    screen::voxel_world::{item::spawn_item, voxel_util::Blocks},
};

use super::{VoxelChunk, VoxelId};

pub struct VoxelLogic;

impl Plugin for VoxelLogic {
    fn build(&self, app: &mut App) {}
}

#[derive(Component)]
struct Extractor;

fn drill_logic(
    selected: Res<HexSelect>,
    extractors: Query<&VoxelId, With<Extractor>>,
    mut commands: Commands,
    voxels: Res<Blocks>,
    chunks: Res<Assets<VoxelChunk>>,
) {
    let Some(chunk) = chunks.get(selected.chunk.id()) else {
        warn!("Chunk Not loaded");
        return;
    };
    for extractor in &extractors {
        // spawn_item(block, voxels, offset, &mut commands)
    }
}
