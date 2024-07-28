//! The screen state for the voxel world game loop.

mod player_controller;
pub mod ui;
pub mod voxel_util;
pub mod world;

mod item;

use super::{inventory::Inventory, Screen};
use crate::game::{assets::SoundtrackKey, audio::soundtrack::PlaySoundtrack};
use bevy::{ecs::system::EntityCommands, input::common_conditions::input_just_pressed, prelude::*};
use player_controller::spawn_player;
use serde::{Deserialize, Serialize};
use serde_big_array::BigArray;
use std::fmt::Debug;
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
    app.init_resource::<Blocks>();
    app.add_plugins(player_controller::VoxelCamera);
    app.register_type::<Inventory>();
    app.add_plugins(item::ItemPlugin);
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

const VOXEL_DIVISION_FACTOR: usize = 2;

#[derive(Asset, Reflect, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub enum BlockType {
    Basic(BasicBlock),
    Complex(ComplexBlock),
}

impl Debug for BlockType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BlockType::Basic(block) => f.write_fmt(format_args!("{:?}", block)),
            BlockType::Complex(block) => f.write_fmt(format_args!("{:?}", block)),
        }
    }
}

#[derive(Asset, Debug, Serialize, Deserialize, Clone, PartialEq, Copy, Eq, Reflect, Hash)]
pub enum BasicBlock {
    Air,
    Stone,
    Coal,
    IronOre,
    IronBlock,
    BedRock,
    Score,
}

#[derive(Asset, Debug, Serialize, Deserialize, Clone, PartialEq, Copy, Eq, Reflect, Hash)]
pub enum ComplexBlock {
    Furnace,
    Drill,
    Voxel(#[serde(with = "BigArray")] [BasicBlock; VOXEL_DIVISION_FACTOR.pow(3)]),
}

// For Multi-Voxel mixing ensure that if 2 voxels can be compressed into a singular one that they are resolved as a  single voxel, not a MultiVoxel

impl BlockType {
    pub fn iter() -> std::slice::Iter<'static, Self> {
        static BLOCKS: [BlockType; 10] = [
            BlockType::Basic(BasicBlock::Air),
            BlockType::Basic(BasicBlock::Stone),
            BlockType::Basic(BasicBlock::Coal),
            BlockType::Basic(BasicBlock::IronOre),
            BlockType::Basic(BasicBlock::IronBlock),
            BlockType::Basic(BasicBlock::BedRock),
            BlockType::Complex(ComplexBlock::Furnace),
            BlockType::Complex(ComplexBlock::Drill),
            BlockType::Complex(ComplexBlock::Voxel(
                [BasicBlock::Air; VOXEL_DIVISION_FACTOR.pow(3)],
            )),
            BlockType::Basic(BasicBlock::Score),
        ];
        BLOCKS.iter()
    }

    pub const fn texture_path(&self) -> &'static str {
        match self {
            BlockType::Basic(block) => block.texture_path(),
            BlockType::Complex(block) => block.texture_path(),
        }
    }

    pub const fn is_solid(&self) -> bool {
        match self {
            BlockType::Basic(block) => block.is_solid(),
            BlockType::Complex(block) => block.is_solid(),
        }
    }

    const fn mesh_path(&self) -> Option<&'static str> {
        match self {
            BlockType::Complex(ComplexBlock::Furnace) => {
                Some("images/multi_blocks/furnace.glb#Mesh0/Primitive0")
            }
            BlockType::Complex(ComplexBlock::Drill) => {
                Some("images/multi_blocks/drill.glb#Mesh1/Primitive0")
            }
            _ => None,
        }
    }

    pub const fn melt(&self) -> Option<BlockType> {
        match self {
            BlockType::Basic(BasicBlock::IronOre) => Some(BlockType::Basic(BasicBlock::IronBlock)),
            _ => None,
        }
    }

    pub const fn fuel(&self) -> bool {
        matches!(self, BlockType::Basic(BasicBlock::Coal))
    }

    pub fn add_components(&self, commands: &mut EntityCommands) {
        match self {
            BlockType::Basic(block) => {
                block.add_components(commands);
            }
            BlockType::Complex(block) => {
                block.add_components(commands);
            }
        }
    }

    pub fn can_mine(&self) -> bool {
        match self {
            BlockType::Basic(BasicBlock::Air) => false,
            BlockType::Basic(_) => true,
            BlockType::Complex(ComplexBlock::Voxel(_)) => false,
            BlockType::Complex(_) => true,
        }
    }
}

impl BasicBlock {
    pub const fn texture_path(&self) -> &'static str {
        match self {
            BasicBlock::Air => "",
            BasicBlock::Stone => "images/voxels/stone.png",
            BasicBlock::Coal => "images/voxels/coal.png",
            BasicBlock::IronOre => "images/voxels/ore_iron.png",
            BasicBlock::IronBlock => "images/voxels/refined_iron.png",
            BasicBlock::BedRock => "images/voxels/bedrock.png",
            BasicBlock::Score => "images/voxels/star.png",
        }
    }

    pub const fn is_solid(&self) -> bool {
        !matches!(self, BasicBlock::Air)
    }

    pub fn add_components(&self, commands: &mut EntityCommands) {
        if self == &BasicBlock::Score {
            commands.insert(world::voxel_logic::ScoreGive);
        }
    }
}

impl ComplexBlock {
    pub const fn texture_path(&self) -> &'static str {
        match self {
            ComplexBlock::Furnace => "images/multi_blocks/furnace.png",
            ComplexBlock::Drill => "images/voxels/refined_iron.png",
            ComplexBlock::Voxel(_) => "",
        }
    }

    pub const fn is_solid(&self) -> bool {
        true
    }

    pub fn add_components(&self, commands: &mut EntityCommands) {
        match self {
            ComplexBlock::Drill => {
                commands.insert(world::voxel_logic::Extractor);
            }
            ComplexBlock::Furnace => {
                commands.insert(world::voxel_logic::Melter);
            }
            _ => {}
        }
    }
}
