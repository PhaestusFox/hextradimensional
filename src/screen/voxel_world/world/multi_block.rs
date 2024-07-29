use bevy::{prelude::*, utils::HashMap};

use crate::game::HexSelect;
use crate::screen::voxel_world::voxels::{Block, Blocks};

use super::{VoxelChunk, VoxelId, CHUNK_SIZE};

use super::super::voxels::BlockType;

#[derive(Resource)]
pub struct MultiBlocks {
    recipes: HashMap<MultiBlockType, MultiBlockRecipe>,
}

impl FromWorld for MultiBlocks {
    fn from_world(_world: &mut World) -> Self {
        let mut map: bevy::utils::hashbrown::HashMap<MultiBlockType, MultiBlockRecipe> =
            HashMap::new();
        map.insert(
            MultiBlockType::Furnace,
            MultiBlockRecipe {
                size: IVec3::new(3, 3, 3),
                rules: vec![
                    MultiBlockRule::Solid,
                    MultiBlockRule::Solid,
                    MultiBlockRule::Solid,
                    MultiBlockRule::Solid,
                    MultiBlockRule::Specific(BlockType::Stone),
                    MultiBlockRule::Solid,
                    MultiBlockRule::Solid,
                    MultiBlockRule::Solid,
                    MultiBlockRule::Solid,
                    MultiBlockRule::Solid,
                    MultiBlockRule::Specific(BlockType::Stone),
                    MultiBlockRule::Solid,
                    MultiBlockRule::Specific(BlockType::Stone),
                    MultiBlockRule::Empty,
                    MultiBlockRule::Specific(BlockType::Stone),
                    MultiBlockRule::Solid,
                    MultiBlockRule::Specific(BlockType::Stone),
                    MultiBlockRule::Solid,
                    MultiBlockRule::Solid,
                    MultiBlockRule::Solid,
                    MultiBlockRule::Solid,
                    MultiBlockRule::Solid,
                    MultiBlockRule::Specific(BlockType::Stone),
                    MultiBlockRule::Solid,
                    MultiBlockRule::Solid,
                    MultiBlockRule::Solid,
                    MultiBlockRule::Solid,
                ],
                output_clear: vec![ClearType::All],
                output_block: MultiOutput::Specific(BlockType::Furnace),
                output_offset: IVec3::new(1, 1, 1),
            },
        );
        map.insert(
            MultiBlockType::Smelt,
            MultiBlockRecipe {
                size: IVec3::new(1, 3, 1),
                rules: vec![
                    MultiBlockRule::Fuel,
                    MultiBlockRule::Specific(BlockType::Furnace),
                    MultiBlockRule::CanMelt,
                ],
                output_block: MultiOutput::Melt(IVec3::new(0, 2, 0)),
                output_offset: IVec3::new(0, 2, 0),
                output_clear: vec![
                    ClearType::Offset(IVec3::ZERO),
                    ClearType::Offset(IVec3::new(0, 2, 0)),
                ],
            },
        );
        map.insert(
            MultiBlockType::Drill,
            MultiBlockRecipe {
                size: IVec3::splat(3),
                rules: vec![
                    MultiBlockRule::Empty,
                    MultiBlockRule::Empty,
                    MultiBlockRule::Empty,
                    MultiBlockRule::Empty,
                    MultiBlockRule::Specific(BlockType::IronBlock),
                    MultiBlockRule::Empty,
                    MultiBlockRule::Empty,
                    MultiBlockRule::Empty,
                    MultiBlockRule::Empty,
                    MultiBlockRule::Empty,
                    MultiBlockRule::Specific(BlockType::IronBlock),
                    MultiBlockRule::Empty,
                    MultiBlockRule::Specific(BlockType::IronBlock),
                    MultiBlockRule::Solid,
                    MultiBlockRule::Specific(BlockType::IronBlock),
                    MultiBlockRule::Empty,
                    MultiBlockRule::Specific(BlockType::IronBlock),
                    MultiBlockRule::Empty,
                    MultiBlockRule::Empty,
                    MultiBlockRule::Empty,
                    MultiBlockRule::Empty,
                    MultiBlockRule::Empty,
                    MultiBlockRule::Solid,
                    MultiBlockRule::Empty,
                    MultiBlockRule::Empty,
                    MultiBlockRule::Empty,
                    MultiBlockRule::Empty,
                ],
                output_block: MultiOutput::Specific(BlockType::Drill(
                    crate::screen::hex_vox_util::MapDirection::Down,
                )),
                output_offset: IVec3 { x: 1, y: 1, z: 1 },
                output_clear: vec![ClearType::All],
            },
        );
        map.insert(
            MultiBlockType::Score,
            MultiBlockRecipe {
                size: IVec3::splat(3),
                rules: vec![
                    MultiBlockRule::Specific(BlockType::IronBlock),
                    MultiBlockRule::Specific(BlockType::IronBlock),
                    MultiBlockRule::Specific(BlockType::IronBlock),
                    MultiBlockRule::Specific(BlockType::IronBlock),
                    MultiBlockRule::Specific(BlockType::IronBlock),
                    MultiBlockRule::Specific(BlockType::IronBlock),
                    MultiBlockRule::Specific(BlockType::IronBlock),
                    MultiBlockRule::Specific(BlockType::IronBlock),
                    MultiBlockRule::Specific(BlockType::IronBlock),
                    MultiBlockRule::Specific(BlockType::IronBlock),
                    MultiBlockRule::Specific(BlockType::IronBlock),
                    MultiBlockRule::Specific(BlockType::IronBlock),
                    MultiBlockRule::Specific(BlockType::IronBlock),
                    MultiBlockRule::Specific(BlockType::CopperBlock),
                    MultiBlockRule::Specific(BlockType::IronBlock),
                    MultiBlockRule::Specific(BlockType::IronBlock),
                    MultiBlockRule::Specific(BlockType::IronBlock),
                    MultiBlockRule::Specific(BlockType::IronBlock),
                    MultiBlockRule::Specific(BlockType::IronBlock),
                    MultiBlockRule::Specific(BlockType::IronBlock),
                    MultiBlockRule::Specific(BlockType::IronBlock),
                    MultiBlockRule::Specific(BlockType::IronBlock),
                    MultiBlockRule::Specific(BlockType::IronBlock),
                    MultiBlockRule::Specific(BlockType::IronBlock),
                    MultiBlockRule::Specific(BlockType::IronBlock),
                    MultiBlockRule::Specific(BlockType::IronBlock),
                    MultiBlockRule::Specific(BlockType::IronBlock),
                ],
                output_block: MultiOutput::Specific(BlockType::Score),
                output_offset: IVec3 { x: 1, y: 1, z: 1 },
                output_clear: vec![ClearType::All],
            },
        );
        map.insert(
            MultiBlockType::Piston,
            MultiBlockRecipe {
                size: IVec3::splat(3),
                rules: vec![
                    MultiBlockRule::Specific(BlockType::IronBlock),
                    MultiBlockRule::Specific(BlockType::IronBlock),
                    MultiBlockRule::Specific(BlockType::IronBlock),
                    MultiBlockRule::Specific(BlockType::IronBlock),
                    MultiBlockRule::Specific(BlockType::CobaltBlock),
                    MultiBlockRule::Specific(BlockType::IronBlock),
                    MultiBlockRule::Specific(BlockType::IronBlock),
                    MultiBlockRule::Specific(BlockType::IronBlock),
                    MultiBlockRule::Specific(BlockType::IronBlock),
                    MultiBlockRule::Specific(BlockType::IronBlock),
                    MultiBlockRule::Specific(BlockType::IronBlock),
                    MultiBlockRule::Specific(BlockType::IronBlock),
                    MultiBlockRule::Specific(BlockType::IronBlock),
                    MultiBlockRule::Specific(BlockType::CobaltBlock),
                    MultiBlockRule::Specific(BlockType::IronBlock),
                    MultiBlockRule::Specific(BlockType::IronBlock),
                    MultiBlockRule::Specific(BlockType::IronBlock),
                    MultiBlockRule::Specific(BlockType::IronBlock),
                    MultiBlockRule::Specific(BlockType::CopperBlock),
                    MultiBlockRule::Specific(BlockType::CopperBlock),
                    MultiBlockRule::Specific(BlockType::CopperBlock),
                    MultiBlockRule::Specific(BlockType::CopperBlock),
                    MultiBlockRule::Specific(BlockType::CopperBlock),
                    MultiBlockRule::Specific(BlockType::CopperBlock),
                    MultiBlockRule::Specific(BlockType::CopperBlock),
                    MultiBlockRule::Specific(BlockType::CopperBlock),
                    MultiBlockRule::Specific(BlockType::CopperBlock),
                ],
                output_block: MultiOutput::Specific(BlockType::Piston(
                    crate::screen::hex_vox_util::MapDirection::Down,
                )),
                output_offset: IVec3 { x: 1, y: 1, z: 1 },
                output_clear: vec![ClearType::All],
            },
        );
        map.insert(
            MultiBlockType::PistonL2,
            MultiBlockRecipe {
                size: IVec3::splat(3),
                rules: vec![
                    MultiBlockRule::Specific(BlockType::IronBlock),
                    MultiBlockRule::Specific(BlockType::IronBlock),
                    MultiBlockRule::Specific(BlockType::IronBlock),
                    MultiBlockRule::Specific(BlockType::IronBlock),
                    MultiBlockRule::Specific(BlockType::CopperBlock),
                    MultiBlockRule::Specific(BlockType::IronBlock),
                    MultiBlockRule::Specific(BlockType::IronBlock),
                    MultiBlockRule::Specific(BlockType::IronBlock),
                    MultiBlockRule::Specific(BlockType::IronBlock),
                    MultiBlockRule::Specific(BlockType::IronBlock),
                    MultiBlockRule::Specific(BlockType::IronBlock),
                    MultiBlockRule::Specific(BlockType::IronBlock),
                    MultiBlockRule::Specific(BlockType::IronBlock),
                    MultiBlockRule::Specific(BlockType::CopperBlock),
                    MultiBlockRule::Specific(BlockType::IronBlock),
                    MultiBlockRule::Specific(BlockType::IronBlock),
                    MultiBlockRule::Specific(BlockType::IronBlock),
                    MultiBlockRule::Specific(BlockType::IronBlock),
                    MultiBlockRule::Specific(BlockType::CobaltBlock),
                    MultiBlockRule::Specific(BlockType::CobaltBlock),
                    MultiBlockRule::Specific(BlockType::CobaltBlock),
                    MultiBlockRule::Specific(BlockType::CobaltBlock),
                    MultiBlockRule::Specific(BlockType::CobaltBlock),
                    MultiBlockRule::Specific(BlockType::CobaltBlock),
                    MultiBlockRule::Specific(BlockType::CobaltBlock),
                    MultiBlockRule::Specific(BlockType::CobaltBlock),
                    MultiBlockRule::Specific(BlockType::CobaltBlock),
                ],
                output_block: MultiOutput::Specific(BlockType::PistonL2(
                    crate::screen::hex_vox_util::MapDirection::Down,
                )),
                output_offset: IVec3 { x: 1, y: 1, z: 1 },
                output_clear: vec![ClearType::All],
            },
        );
        MultiBlocks { recipes: map }
    }
}

enum MultiBlockRule {
    Solid,
    Specific(BlockType),
    Empty,
    CanMelt,
    Fuel,
}

enum MultiOutput {
    Specific(BlockType),
    Melt(IVec3),
}

impl MultiBlockRule {
    fn applies_to(&self, block: &Block) -> bool {
        match self {
            MultiBlockRule::Solid => block.is_solid(),
            MultiBlockRule::Specific(cmp) => block.get_type() == *cmp,
            MultiBlockRule::Empty => block.get_type() == BlockType::Air,
            MultiBlockRule::CanMelt => block.melt().is_some(),
            MultiBlockRule::Fuel => block.is_fuel(),
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
enum MultiBlockType {
    Furnace,
    Smelt,
    Drill,
    Score,
    Piston,
    PistonL2,
}

struct MultiBlockRecipe {
    size: IVec3,
    rules: Vec<MultiBlockRule>,
    output_block: MultiOutput,
    output_offset: IVec3,
    output_clear: Vec<ClearType>,
}

impl MultiBlockRecipe {
    pub fn clear(
        &self,
        pos: IVec3,
        chunk: &mut VoxelChunk,
        commands: &mut Commands,
        blocks: &Query<(Entity, &VoxelId)>,
    ) {
        for clear in self.output_clear.iter() {
            clear.apply(self, pos, chunk, commands, blocks);
        }
    }

    pub fn output(
        &self,
        pos: IVec3,
        chunk: &VoxelChunk,
        voxel_data: &Assets<Block>,
        voxels: &Blocks,
    ) -> Vec<(IVec3, BlockType)> {
        let mut out = Vec::new();
        match &self.output_block {
            MultiOutput::Specific(block) => out.push((pos + self.output_offset, block.clone())),
            MultiOutput::Melt(offset) => {
                let block = chunk.get(pos + *offset);
                let block = voxels.get(block);
                let block = voxel_data.get(block.id()).expect("all blocks loaded");
                if let Some(melt) = block.melt() {
                    out.push((pos + *offset, melt));
                }
            }
        };
        out
    }
}

impl ClearType {
    fn apply(
        &self,
        recipe: &MultiBlockRecipe,
        pos: IVec3,
        chunk: &mut VoxelChunk,
        commands: &mut Commands,
        blocks: &Query<(Entity, &VoxelId)>,
    ) {
        match self {
            ClearType::All => {
                for rx in 0..recipe.size.x {
                    for ry in 0..recipe.size.y {
                        for rz in 0..recipe.size.z {
                            let pos = pos + IVec3::new(rx, ry, rz);
                            chunk.set(pos, BlockType::Air);
                            for (entity, id) in blocks {
                                if id.0 == pos {
                                    commands.entity(entity).despawn_recursive();
                                }
                            }
                        }
                    }
                }
            }
            ClearType::Offset(offset) => {
                let pos = pos + *offset;
                chunk.set(pos, BlockType::Air);
                for (entity, id) in blocks {
                    if id.0 == pos {
                        commands.entity(entity).despawn_recursive();
                    }
                }
            }
        }
    }
}

enum ClearType {
    All,
    Offset(IVec3),
}

pub fn check_for_multi_blocks(
    mut chunk_data: ResMut<Assets<VoxelChunk>>,
    blocks: Query<(Entity, &VoxelId)>,
    recipes: Res<MultiBlocks>,
    selected: Res<HexSelect>,
    voxels: Res<Blocks>,
    voxel_data: Res<Assets<Block>>,
    mut commands: Commands,
) {
    let Some(chunk) = chunk_data.get_mut(selected.chunk.id()) else {
        error!("chunk not loaded");
        return;
    };
    for (_, recipe) in recipes.recipes.iter() {
        for x in 0..CHUNK_SIZE as i32 {
            for y in 0..CHUNK_SIZE as i32 {
                'failed: for z in 0..CHUNK_SIZE as i32 {
                    for rx in 0..recipe.size.x {
                        for ry in 0..recipe.size.y {
                            for rz in 0..recipe.size.z {
                                let pos = IVec3::new(x, y, z) + IVec3::new(rx, ry, rz);
                                let rule_index =
                                    rx + rz * recipe.size.x + ry * recipe.size.x * recipe.size.z;
                                let block = chunk.get(pos);
                                let block = voxels.get(block);
                                let block = voxel_data.get(block.id()).expect("all BLocks loaded");
                                if !recipe.rules[rule_index as usize].applies_to(block) {
                                    continue 'failed;
                                }
                            }
                        }
                    }
                    let out = recipe.output(IVec3::new(x, y, z), chunk, &voxel_data, &voxels);
                    recipe.clear(IVec3::new(x, y, z), chunk, &mut commands, &blocks);
                    for (pos, block) in out {
                        let data = voxels.get(block.clone());
                        let data = voxel_data.get(data.id()).expect("all block loaded");
                        let mut entity = commands.spawn((
                            Name::new("Multi Block"),
                            VoxelId(pos),
                            PbrBundle {
                                mesh: data.mesh(),
                                material: data.material(),
                                transform: Transform::from_translation(pos.as_vec3()),
                                ..Default::default()
                            },
                        ));
                        if data.is_solid() {
                            entity.insert(bevy_rapier3d::prelude::Collider::cuboid(0.5, 0.5, 0.5));
                        }
                        chunk.set(pos, block);
                    }
                    println!("Found multi block");
                }
            }
        }
    }
}
