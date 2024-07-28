//! The game's main screen states and transitions between them.

mod credits;
mod hex_map;
pub mod hex_vox_util;
pub mod inventory;
mod loading;
mod splash;
mod title;
pub mod voxel_world;

use bevy::prelude::*;
use voxel_world::BlockType;

pub(super) fn plugin(app: &mut App) {
    app.init_state::<Screen>();
    app.enable_state_scoped_entities::<Screen>();

    app.init_resource::<Score>();
    app.insert_resource(Target(BlockType::Basic(voxel_world::BasicBlock::Stone)));
    app.init_resource::<NextTarget>();

    app.add_plugins((
        splash::plugin,
        loading::plugin,
        title::plugin,
        credits::plugin,
        hex_map::plugin,
        voxel_world::plugin,
    ));
}

#[derive(Resource, Default)]
pub struct Score(i32);

#[derive(Resource)]
pub struct Target(BlockType);

#[derive(Resource, Default)]
pub struct NextTarget(i32);

impl NextTarget {
    pub fn next(&mut self) -> BlockType {
        let out = match self.0 {
            0 => BlockType::Basic(voxel_world::BasicBlock::Coal),
            1 => BlockType::Basic(voxel_world::BasicBlock::IronOre),
            2 => BlockType::Complex(voxel_world::ComplexBlock::Furnace),
            3 => BlockType::Basic(voxel_world::BasicBlock::IronBlock),
            _ => todo!(),
        };
        self.0 += 1;
        out
    }
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
