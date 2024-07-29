use bevy::prelude::*;

use rand::Rng;
use serde::{Deserialize, Serialize};

use super::voxels::BlockType;

/// This describes the main player in the voxel world
#[derive(Component)]
pub struct VoxelPlayer;

#[derive(
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    Default,
    Debug,
    Clone,
    Copy,
    Hash,
    strum_macros::EnumIter,
    Component,
)]
pub enum WorldType {
    #[default]
    Empty,
    Stone,
    Coal,
    Iron,
    Sand,
    Cobalt,
    Copper,
    Potassium,
}

impl WorldType {
    pub fn sample(&self, rng: &mut impl Rng, pos: IVec3) -> BlockType {
        if pos.y == -1 {
            return BlockType::BedRock;
        }
        match self {
            WorldType::Empty => BlockType::Air,
            WorldType::Stone => BlockType::Stone,
            WorldType::Coal => {
                if rng.gen_bool(0.25) {
                    BlockType::Coal
                } else {
                    BlockType::Stone
                }
            }
            WorldType::Iron => {
                if rng.gen_bool(0.25) {
                    BlockType::IronOre
                } else {
                    BlockType::Stone
                }
            }
            WorldType::Sand => {
                if pos.y > 10 {
                    BlockType::Air
                } else if pos.y == 10 && rng.gen_bool(0.90) {
                    BlockType::Air
                } else if pos.y == 9 && rng.gen_bool(0.60) {
                    BlockType::Air
                } else if pos.y == 8 && rng.gen_bool(0.30) {
                    BlockType::Air
                } else if pos.y == 7 && rng.gen_bool(0.10) {
                    BlockType::Air
                } else {
                    BlockType::Sand
                }
            }
            WorldType::Cobalt => {
                if rng.gen_bool(0.25) {
                    BlockType::CobaltOre
                } else {
                    BlockType::Stone
                }
            }
            WorldType::Copper => {
                if rng.gen_bool(0.5 - (0.4 / 16.) * pos.y as f64 + 0.1) {
                    BlockType::CopperOre
                } else {
                    BlockType::Stone
                }
            }
            WorldType::Potassium => {
                let one = rng.gen_range(0.0..(1.0 - (0.9 / 16.) * pos.y as f64 + 0.1));
                let two = rng.gen_range(0.0..((0.9 / 16.) * pos.y as f64 + 0.1));
                if one > 0.5 && one > two {
                    BlockType::Magnesium
                } else if two > 0.5 && two < one {
                    BlockType::Sodium
                } else if two < 0.5 && one > two {
                    BlockType::Potassium
                } else {
                    use rand::seq::SliceRandom;
                    [
                        BlockType::Magnesium,
                        BlockType::Sodium,
                        BlockType::Potassium,
                        BlockType::Magnesium,
                    ]
                    .choose(rng)
                    .cloned()
                    .unwrap_or(BlockType::Magnesium)
                }
            }
        }
    }
}

// #[derive(Resource)]
// pub struct BlocksOld {
//     meshs: HashMap<BlockType, Handle<Mesh>>,
//     textures: HashMap<BlockType, Handle<StandardMaterial>>,
// }

// impl BlocksOld {
//     pub fn texture(&self, block: &BlockType) -> Handle<StandardMaterial> {
//         self.textures.get(block).cloned().unwrap_or_default()
//     }
//     pub fn mesh(&self, block: &BlockType) -> Handle<Mesh> {
//         self.meshs.get(block).cloned().unwrap_or_default()
//     }
// }

// impl FromWorld for BlocksOld {
//     fn from_world(world: &mut World) -> Self {
//         let mut blocks = BlocksOld {
//             meshs: HashMap::default(),
//             textures: HashMap::default(),
//         };
//         let asset_server = world.resource::<AssetServer>().clone();
//         world.resource_scope::<Assets<StandardMaterial>, ()>(|world, mut materials| {
//             let mut meshes = world.resource_mut::<Assets<Mesh>>();
//             let default = meshes.add(Cuboid::from_length(1.));
//             for block in BlockType::iter() {
//                 let texture_path = block.texture_path();
//                 let mesh_path = block.mesh_path();
//                 blocks.textures.insert(
//                     block.clone(),
//                     materials.add(StandardMaterial {
//                         base_color_texture: Some(asset_server.load(texture_path)),
//                         ..Default::default()
//                     }),
//                 );
//                 if let Some(mesh) = mesh_path {
//                     blocks.meshs.insert(*block, asset_server.load(mesh));
//                 } else {
//                     blocks.meshs.insert(*block, default.clone());
//                 }
//             }
//         });
//         blocks
//     }
// }
