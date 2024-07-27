//! The game's main screen states and transitions between them.

mod credits;
mod hex_map;
mod loading;
mod splash;
mod title;
pub mod voxel_world;

use bevy::prelude::*;
use hex_map::cells::HexId;
use voxel_world::world::VoxelChunk;

pub(super) fn plugin(app: &mut App) {
    app.init_state::<Screen>();
    app.enable_state_scoped_entities::<Screen>();

    app.add_plugins((
        splash::plugin,
        loading::plugin,
        title::plugin,
        credits::plugin,
        hex_map::plugin,
        voxel_world::plugin,
    ));

    app.insert_resource(HexSelect {
        hex_id: HexId::new(0, 0),
        direction: MapDirection::Up,
        world: voxel_world::voxel_util::WorldType::Empty,
        chunk: Handle::default(),
    });
}

/// The game's main screen states.
#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Default)]
pub enum Screen {
    #[default]
    Splash,
    Loading,
    Title,
    Credits,
    HexMap,
    VoxelWorld,
    //Multiplayer,
}

/// This represents the edges of the hexagon mapping to the voxel world.
/// The Direction with reference to the hexagon is in clockwise order for the enum, starting from the top edge.
#[derive(Clone, Copy, PartialEq, strum_macros::EnumIter, Debug, Component, Eq, Hash, Reflect)]
pub enum MapDirection {
    Up,
    North,
    East,
    Down,
    South,
    West,
}

/// The current selected hexagon
#[derive(Resource, Debug)]
pub struct HexSelect {
    pub hex_id: hex_map::cells::HexId,
    pub direction: MapDirection,
    pub world: voxel_world::voxel_util::WorldType,
    pub chunk: Handle<VoxelChunk>,
}
