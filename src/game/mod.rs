pub mod assets;
pub mod audio;
pub mod main_character;
pub mod save;

use crate::screen::{
    hex_vox_util::{HexId, MapDirection},
    voxel_world::{self, world::VoxelChunk},
};
use bevy::{
    app::{App, Startup},
    asset::Handle,
    prelude::{Component, IntoSystemConfigs, Resource},
    reflect::Reflect,
};
use main_character::spawn_main_player;
use save::inventory_load;

///Loaded
pub(super) fn plugin(app: &mut App) {
    app.add_plugins((audio::plugin, assets::plugin, save::plugin));
    app.insert_resource(HexSelect {
        hex_id: HexId::new(0, 0),
        direction: MapDirection::Up,
        world: voxel_world::voxel_util::WorldType::Empty,
        chunk: Handle::default(),
    });
    app.add_systems(
        Startup,
        (spawn_main_player, inventory_load.after(spawn_main_player)),
    );
}

/// The current selected hexagon
#[derive(Resource, Debug)]
pub struct HexSelect {
    pub hex_id: HexId,
    pub direction: MapDirection,
    pub world: voxel_world::voxel_util::WorldType,
    pub chunk: Handle<VoxelChunk>,
}

#[derive(
    Debug,
    leafwing_input_manager::Actionlike,
    Reflect,
    Clone,
    Copy,
    Hash,
    PartialEq,
    Eq,
    strum::EnumIter,
    Component,
)]
#[non_exhaustive]
pub enum PlayerAction {
    Hit,
    Place,
    Jump,
    Move,
    Look,
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    EnterHex,
    ExitChunk,
    ToolbarNext,
    ToolbarPrev,
    ItemInc,
    ItemDec,
}
