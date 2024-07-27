pub mod assets;
pub mod audio;
pub mod main_character;

use crate::screen::{
    hex_vox_util::{HexId, MapDirection},
    voxel_world::{self, world::VoxelChunk},
};
use bevy::{app::App, asset::Handle, prelude::Resource};
use main_character::Seed;

///Loaded
pub(super) fn plugin(app: &mut App) {
    app.add_plugins((audio::plugin, assets::plugin));
    app.insert_resource(HexSelect {
        hex_id: HexId::new(0, 0),
        direction: MapDirection::Up,
        world: voxel_world::voxel_util::WorldType::Empty,
        chunk: Handle::default(),
    });
    app.insert_resource(Seed(0u64));
}

/// The current selected hexagon
#[derive(Resource, Debug)]
pub struct HexSelect {
    pub hex_id: HexId,
    pub direction: MapDirection,
    pub world: voxel_world::voxel_util::WorldType,
    pub chunk: Handle<VoxelChunk>,
}
