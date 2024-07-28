use crate::screen::{hex_vox_util::HexId, voxel_world::player_controller::VoxelCamera, Screen};
use bevy::{prelude::*, utils::HashMap};
use bevy_rapier3d::prelude::*;
use rand::{Rng, SeedableRng};
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;

use super::{BasicBlock, BlockType, ComplexBlock};

pub struct VoxelPlugin;

impl Plugin for VoxelPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Blocks>().add_plugins(VoxelCamera);
    }
}

/// This describes the main player in the voxel world
#[derive(Component)]
pub struct VoxelPlayer;

#[derive(PartialEq, Eq, Serialize, Deserialize, Default, Debug, Clone, Copy)]
pub enum WorldType {
    #[default]
    Empty,
    Stone,
    Coal,
    Iron,
}

#[derive(Resource)]
pub struct Solid([bool; 16 * 16 * 16]);

impl Default for Solid {
    fn default() -> Self {
        Self([false; 16 * 16 * 16])
    }
}

impl Solid {
    fn set(&mut self, x: i32, y: i32, z: i32, val: bool) {
        self.0[(x + z * 16 + y * 16 * 16) as usize] = val;
    }
    fn clear(&mut self) {
        self.0 = [false; 16 * 16 * 16];
    }
    pub fn get(&self, x: i32, y: i32, z: i32) -> bool {
        self.0
            .get((x + z * 16 + y * 16 * 16) as usize)
            .cloned()
            .unwrap_or(false)
    }
}

impl WorldType {
    fn from_u8(id: u8) -> WorldType {
        match id {
            0 => WorldType::Empty,
            1 => WorldType::Stone,
            2 => WorldType::Coal,
            3 => WorldType::Iron,
            _ => unreachable!(),
        }
    }

    pub fn sample(&self, rng: &mut impl Rng, pos: IVec3) -> BlockType {
        if pos.y == -1 {
            return BlockType::Basic(BasicBlock::BedRock);
        }
        match self {
            WorldType::Empty => BlockType::Basic(BasicBlock::Air),
            WorldType::Stone => BlockType::Basic(BasicBlock::Stone),
            WorldType::Coal => {
                if rng.gen_bool(0.25) {
                    BlockType::Basic(BasicBlock::Coal)
                } else {
                    BlockType::Basic(BasicBlock::Stone)
                }
            }
            WorldType::Iron => {
                if rng.gen_bool(0.25) {
                    BlockType::Basic(BasicBlock::IronOre)
                } else {
                    BlockType::Basic(BasicBlock::Stone)
                }
            }
        }
    }
}

#[derive(Resource)]
pub struct Blocks {
    meshs: HashMap<BlockType, Handle<Mesh>>,
    textures: HashMap<BlockType, Handle<StandardMaterial>>,
}

impl Blocks {
    pub fn texture(&self, block: &BlockType) -> Handle<StandardMaterial> {
        self.textures.get(block).cloned().unwrap_or_default()
    }
    pub fn mesh(&self, block: &BlockType) -> Handle<Mesh> {
        self.meshs.get(block).cloned().unwrap_or_default()
    }
}

impl FromWorld for Blocks {
    fn from_world(world: &mut World) -> Self {
        let mut blocks = Blocks {
            meshs: HashMap::default(),
            textures: HashMap::default(),
        };
        let asset_server = world.resource::<AssetServer>().clone();
        world.resource_scope::<Assets<StandardMaterial>, ()>(|world, mut materials| {
            let mut meshes = world.resource_mut::<Assets<Mesh>>();
            let default = meshes.add(Cuboid::from_length(1.));
            for block in BlockType::iter() {
                let texture_path = block.texture_path();
                let mesh_path = block.mesh_path();
                blocks.textures.insert(
                    block.clone(),
                    materials.add(StandardMaterial {
                        base_color_texture: Some(asset_server.load(texture_path)),
                        ..Default::default()
                    }),
                );
                if let Some(mesh) = mesh_path {
                    blocks.meshs.insert(*block, asset_server.load(mesh));
                } else {
                    blocks.meshs.insert(*block, default.clone());
                }
            }
        });
        blocks
    }
}
