use bevy::prelude::*;

use crate::{
    game::HexSelect,
    screen::{
        voxel_world::{item::spawn_item, voxel_util::Blocks, BlockType},
        Screen,
    },
};

use super::{VoxelChunk, VoxelId};

pub struct VoxelLogic;

impl Plugin for VoxelLogic {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            drill_logic.run_if(in_state(Screen::VoxelWorld)),
        );
    }
}

#[derive(Component)]
pub struct Extractor;

fn drill_logic(
    selected: Res<HexSelect>,
    extractors: Query<&VoxelId, With<Extractor>>,
    mut commands: Commands,
    voxels: Res<Blocks>,
    chunks: Res<Assets<VoxelChunk>>,
) {
    println!("Extracting");
    let Some(chunk) = chunks.get(selected.chunk.id()) else {
        warn!("Chunk Not loaded");
        return;
    };
    for extractor in &extractors {
        let below = chunk.get(extractor.0 - IVec3::Y);
        if !below.can_mine() {
            return;
        }
        spawn_item(
            below,
            &voxels,
            (extractor.0 + IVec3::Y).as_vec3(),
            &mut commands,
        )
    }
}
