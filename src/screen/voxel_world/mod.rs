//! The screen state for the voxel world game loop.
pub mod inventory;
mod player_controller;
pub mod ui;
pub mod voxel_util;
pub mod world;

use super::{MapDirection, Screen};
use crate::game::{assets::SoundtrackKey, audio::soundtrack::PlaySoundtrack};
use bevy::{input::common_conditions::input_just_pressed, prelude::*};
use inventory::Inventory;
use player_controller::spawn_player;
use std::sync::Arc;
use ui::{
    cleanup_inventory_ui, handle_slot_selection, setup_inventory_ui, toggle_full_inventory,
    update_inventory_ui,
};
use voxel_util::Blocks;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        OnEnter(Screen::VoxelWorld),
        (
            enter_playing,
            spawn_player,
            setup_inventory_ui.after(spawn_player),
        ),
    );
    app.add_systems(
        Update,
        (update_inventory_ui, handle_slot_selection).run_if(in_state(Screen::VoxelWorld)),
    );
    app.add_systems(
        OnExit(Screen::VoxelWorld),
        (exit_playing, cleanup_inventory_ui),
    );
    //.add_systems(Update, update_inventory_ui);

    app.add_systems(
        Update,
        return_to_hex_map
            .run_if(in_state(Screen::VoxelWorld).and_then(input_just_pressed(KeyCode::Escape))),
    );
    app.add_systems(
        Update,
        toggle_full_inventory
            .run_if(in_state(Screen::VoxelWorld).and_then(input_just_pressed(KeyCode::KeyT))),
    );
    app.add_systems(Update, update_inventory_ui);
    app.init_resource::<Blocks>();
    app.add_plugins(player_controller::VoxelCamera);
    app.register_type::<Inventory>();
    world::voxel_world(app);
}

fn enter_playing(mut commands: Commands) {
    commands.trigger(PlaySoundtrack::Key(SoundtrackKey::Gameplay));
}

fn exit_playing(mut commands: Commands) {
    // We could use [`StateScoped`] on the sound playing entites instead.
    commands.trigger(PlaySoundtrack::Disable);
}

fn return_to_hex_map(mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::HexMap);
}

const VOXEL_DIVISION_FACTOR: usize = 16;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Reflect)]
pub struct VoxelData(Arc<[BlockType; VOXEL_DIVISION_FACTOR.pow(3)]>);

impl Default for VoxelData {
    fn default() -> Self {
        VoxelData(Arc::new(
            [const { BlockType::Air }; VOXEL_DIVISION_FACTOR.pow(3)],
        ))
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Reflect)]
pub struct DirectedVoxel {
    direction: Option<MapDirection>,
    voxel: VoxelData,
}

/// All block types
#[derive(Debug, Hash, PartialEq, Eq, strum_macros::EnumIter, Clone, Reflect, Component)]
pub enum BlockType {
    Air,
    Stone,
    Coal,
    IronOre,
    IronBlock,
    Furnace,
    Drill,
    Voxel(VoxelData),
    MultiVoxel(Vec<DirectedVoxel>),
}

// For Multi-Voxel mixing ensure that if 2 voxels can be compressed into a singular one that they are resolved as a  single voxel, not a MultiVoxel

impl BlockType {
    pub const fn texture_path(&self) -> &'static str {
        match self {
            BlockType::Air => "", // ! To fix
            BlockType::Stone => "images/voxels/stone.png",
            BlockType::Coal => "images/voxels/coal.png",
            BlockType::Voxel(_) => "",
            BlockType::MultiVoxel(_) => "",
            BlockType::IronBlock => "images/voxels/refined_iron.png",
            BlockType::IronOre => "images/voxels/ore_iron.png",
            BlockType::Furnace => "images/multi_blocks/furnace.png",
            BlockType::Drill => "images/voxels/refined_iron.png",
        }
    }

    pub const fn is_solid(&self) -> bool {
        match self {
            BlockType::Air => false,
            BlockType::Stone => true,
            BlockType::Coal => true,
            BlockType::Voxel(_) => false,
            BlockType::MultiVoxel(_) => false,
            BlockType::IronOre => true,
            BlockType::IronBlock => true,
            BlockType::Furnace => true,
            BlockType::Drill => true,
        }
    }

    const fn mesh_path(&self) -> Option<&'static str> {
        match self {
            BlockType::Air => None,
            BlockType::Stone => None,
            BlockType::Coal => None,
            BlockType::Voxel(_) => None,
            BlockType::MultiVoxel(_) => None,
            BlockType::IronOre => None,
            BlockType::IronBlock => None,
            BlockType::Furnace => Some("images/multi_blocks/furnace.glb#Mesh0/Primitive0"),
            BlockType::Drill => Some("images/multi_blocks/drill.glb#Mesh1/Primitive0"),
        }
    }

    pub const fn melt(&self) -> Option<BlockType> {
        match self {
            BlockType::Air => None,
            BlockType::Stone => None,
            BlockType::Coal => None,
            BlockType::IronOre => Some(BlockType::IronBlock),
            BlockType::IronBlock => None,
            BlockType::Furnace => None,
            BlockType::Voxel(_) => None,
            BlockType::MultiVoxel(_) => None,
            BlockType::Drill => None,
        }
    }

    pub const fn fuel(&self) -> bool {
        match self {
            BlockType::Air => false,
            BlockType::Stone => false,
            BlockType::Coal => true,
            BlockType::IronOre => false,
            BlockType::IronBlock => false,
            BlockType::Furnace => false,
            BlockType::Drill => false,
            BlockType::Voxel(_) => false,
            BlockType::MultiVoxel(_) => false,
        }
    }
}
